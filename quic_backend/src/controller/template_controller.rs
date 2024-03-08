use uuid::Uuid;

use crate::*;

use self::{compressor::{compress_and_save, retrieve_decompress}, 
  file::gen_filename, stage_dto::{StageTrait, SubmitStage}, 
  template_dto::{to_nlist, SubmitGetTemplate, SubmitTemplate, TemplateTrait}
};

// =================================================
// GET
pub(crate) fn get_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();

  let data = get_data(data_path, submit.filename.clone());
  if data.is_err() { return Err(data.unwrap_err()); }

  Ok(Some(data.unwrap().to_string()))
}

pub(crate) fn get_template_nlist(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();
  // if submit.filename.is_none() { return Err("Filename must be defined.".to_owned()); }

  let data = get_data(data_path, submit.filename.clone());
  if data.is_err() { return Err(data.unwrap_err()); }
  let retval = serde_json::to_string(&to_nlist(data.unwrap()));
  if retval.is_err() { return Err(retval.unwrap_err().to_string()); }

  Ok(Some(retval.unwrap()))
}


// =============================================
// POST and PUT
/// Create new template. We don't know the filename and uuid yet. 
pub(crate) fn new_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitTemplate = serde_json::from_slice(&msg).unwrap();
  
  let uuid = Uuid::now_v7().to_string();
  let data = submit.new_template(uuid.clone());

  let mut filename = gen_filename(submit.name.clone());
  // Move these into gen_filename later. 
  if filename.len() == 0 { filename = "untitled".to_owned(); }
  filename.push_str("_");
  filename.push_str(&uuid);
  filename.push_str(".json.zl");
  
  // Will not create file but use compress_and_save in the future. 
  let mut data_path = data_path;
  data_path.push("template");
  let ret = compress_and_save(data.to_string(), data_path, filename.clone());
  if ret.is_err() { return Err(ret.unwrap_err()); }

  Ok(Some(filename))
}

pub(crate) fn edit_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let data_path = modify_datapath(data_path);
  
  // Edit Template (Name and Description)
  let submit: SubmitTemplate = serde_json::from_slice(&msg).unwrap();

  let old_serde = retrieve_decompress(
    data_path.clone(), submit.filename.clone().unwrap());
  if old_serde.is_err() { return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let edited_serde = submit.edit_template(old_serde);
  // Savings occur after we edit stage. 

  // Edit Stage
  let submit: SubmitStage = serde_json::from_slice(&msg).unwrap();
  
  let new_serde = submit.edit_stage(edited_serde);
  let ret = compress_and_save(new_serde.to_string(), data_path.clone(), submit.filename.clone());
  if ret.is_err() { return Err(ret.unwrap_err()); }

  // We'll update to change filename too in the future. That isn't too important for now. 
  Ok(Some(new_serde.to_string()))
}


// pub(crate) fn save_reminder(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
//   let mut data_path = data_path;
//   data_path.push("template");

//   let submit: SubmitReminder = serde_json::from_slice(&msg).unwrap();

//   let old_serde = retrieve_decompress(
//     data_path.clone(), submit.filename.clone());
//   if old_serde.is_err() { return Err(old_serde.unwrap_err()); }
//   let old_serde = old_serde.unwrap();

//   let edited_serde = submit.to_serde(old_serde);
//   if edited_serde.is_none() { return Err("There's an error with to_serde reminder_dto.".to_owned()); }
//   let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), data_path.clone(), submit.filename.clone());
//   if ret.is_err() { return Err(ret.unwrap_err()); }

//   Ok(Some(edited_serde.unwrap().to_string()))
// }


// ================================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  let mut data_path = data_path;
  data_path.push("template");
  data_path
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}