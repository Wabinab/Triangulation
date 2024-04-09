use uuid::Uuid;
use std::fs;

use crate::{messages::FILENAME_NO_NULL, *};

use self::{compressor::{compress_and_save, retrieve_decompress, retrieve_decompress_fullpath}, file::gen_filename, filelist_dto::SubmitFileList, stage_dto::{StageTrait, SubmitStage}, template_dto::{to_basic_template, to_nameonly, to_nlist_temp, SubmitCloneTemp, SubmitGetTemplate, SubmitTemplate, SubmitTemplateVer, TemplateTrait, TemplateVerTrait}, versioning::{get_verpath, upd_ver_temp}
};

// =================================================
// GET
// pub(crate) fn get_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
//   let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();

//   let data = get_data(data_path, submit.filename.clone());
//   if data.is_err() { return Err(data.unwrap_err()); }

//   Ok(Some(data.unwrap().to_string()))
// }

pub(crate) fn get_template_nlist(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();

  let data = get_data(data_path, submit.filename.clone());
  if data.is_err() { error!("get_template_nlist data"); return Err(data.unwrap_err()); }
  let retval = serde_json::to_string(
    &to_nlist_temp(data.unwrap()));
  if retval.is_err() { error!("get_template_nlist retval"); return Err(retval.unwrap_err().to_string()); }

  Ok(Some(retval.unwrap()))
}

/// Although called 'nameonly', but actually returns name and uuid. 
/// This return a list of templates rather than one single template
pub(crate) fn get_templates_nameonly(data_path: PathBuf) -> Result<Option<String>, String> {
  let paths = fs::read_dir(modify_datapath(data_path)).unwrap();
  let mut errors: Vec<String> = vec![]; 
  let mut retvals: Vec<Value> = vec![];
  for path in paths {
    if path.is_err() { error!("get_templates_nameonly path as_ref"); errors.push(path.unwrap_err().to_string()); continue; }
    let data = retrieve_decompress_fullpath(path.unwrap().path());
    if data.is_err() { error!("get_templates_nameonly retrieve data"); errors.push(data.unwrap_err()); continue; }
    let retval = serde_json::to_value(&to_nameonly(data.unwrap()));
    if retval.is_err() { error!("get_templates_nameonly retval"); errors.push(retval.unwrap_err().to_string()); continue; }
    retvals.push(retval.unwrap());
  }
  Ok(Some(json!({
    "data": retvals,
    "err": errors
  }).to_string()))
}

/// Get the current version for this template
pub(crate) fn get_template_version(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitTemplateVer = serde_json::from_slice(&msg).unwrap();
  let res = submit.get_version(data_path);
  if res.is_err() { error!("get_template_version res"); return Err(res.unwrap_err()); }
  Ok(Some(json!({
    "version": res.unwrap()
  }).to_string()))
}

/// Apart from name and uuid, also return description. 
/// This is use for display list in frontend. 
pub(crate) fn get_templates(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitFileList = serde_json::from_slice(&msg).unwrap();
  let paths: Vec<_> = fs::read_dir(modify_datapath(data_path)).unwrap().collect();
  let mut errors: Vec<String> = vec![]; 
  let mut retvals: Vec<Value> = vec![];
  for path in paths
    .iter()
    .skip(submit.page_no * submit.page_size)
    .take(submit.page_size) 
  {
    let path = path.as_ref();
    if path.is_err() { error!("get_templates path as_ref"); errors.push(path.unwrap_err().to_string()); continue; }
    let data = retrieve_decompress_fullpath(path.unwrap().path());
    if data.is_err() { error!("get_templates retrieve temp"); errors.push(data.unwrap_err()); continue; }
    let retval = serde_json::to_value(&to_basic_template(data.unwrap()));
    if retval.is_err() { error!("get_templates retval"); errors.push(retval.unwrap_err().to_string()); continue; }
    retvals.push(retval.unwrap());
  }

  Ok(Some(json!({
    "total_count": paths.len(),
    "data": retvals,
    "err": errors
  }).to_string()))
}

// =============================================
// POST and PUT
/// Create new template. We don't know the filename and uuid yet. 
pub(crate) fn new_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitTemplate = serde_json::from_slice(&msg).unwrap();
  
  let uuid = Uuid::now_v7().to_string();
  let filename = gen_filename(TEMPLATE_NAME.to_string(), uuid.clone(), None);

  let data = submit.new_template(uuid.clone());
  let data_path = modify_datapath(data_path);
  let ret = compress_and_save(
    data.to_string(), data_path, filename.clone());
  if ret.is_err() { error!("new_template ret"); return Err(ret.unwrap_err()); }

  Ok(Some(json!({
    "filename": filename
  }).to_string()))
}


pub(crate) fn edit_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  // Edit Template (Name and Description)
  let submit: SubmitTemplate = serde_json::from_slice(&msg).unwrap();
  if submit.filename.is_none() { 
    error!("edit_template filename template"); 
    return Err(FILENAME_NO_NULL.to_owned()); 
  }

  let old_serde = get_data(
    data_path.clone(), submit.filename.clone().unwrap());
  if old_serde.is_err() { error!("edit_template old_serde"); return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let edited_serde = submit.edit_template(old_serde);
  // Savings occur after we edit stage. 

  // Edit Stage
  let submit: SubmitStage = serde_json::from_slice(&msg).unwrap();

  let new_serde = submit.edit_stage(edited_serde);
  let ret = compress_and_save(
    new_serde.to_string(), modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("edit_template save new serde"); return Err(ret.unwrap_err()); }

  // Update versioning when update template. 
  let ret = upd_ver_temp(get_verpath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("edit_template update version"); return Err(ret.unwrap_err()); }

  // We'll update to change filename too in the future. That isn't too important for now. 
  Ok(Some(new_serde.to_string()))
}

pub(crate) fn delete_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitCloneTemp = serde_json::from_slice(&msg).unwrap();
  let filename = gen_filename(TEMPLATE_NAME.to_owned(), submit.uuid, None);
  let mut filepath = modify_datapath(data_path);
  filepath.push(filename.clone());
  let ret = fs::remove_file(filepath.as_path());
  if ret.is_err() { error!("delete_template failed to remove file."); return Err(ret.unwrap_err().to_string()); }
  Ok(Some(json!({
    "msg": format!("Successfully delete template with filename: {}", filename)
  }).to_string()))
}

pub(crate) fn clone_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitCloneTemp = serde_json::from_slice(&msg).unwrap();
  let filename = gen_filename(TEMPLATE_NAME.to_owned(), submit.uuid, None);
  let data = clone::clone_template(data_path, filename);
  if data.is_err() { error!("template_controller clone_template err."); return Err(data.unwrap_err()); }
  Ok(Some(data.unwrap().to_string()))
}


// ================================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  file::modify_datapath(data_path, "template")
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}