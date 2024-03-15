use uuid::Uuid;

use crate::*;

use self::{compressor::{compress_and_save, retrieve_decompress}, file::gen_filename, project_dto::{to_nlist_proj, ProjectTrait, SubmitGetProject, SubmitProject}, template_dto::to_nlist_temp, versioning::{get_savepath, get_verpath, upd_ver_proj}};

// ===========================================
// GET
/// Not only get the project data, but also its corresponding template nlist. 
/// This is already an "nlist", because we don't need a separate one. 
pub(crate) fn get_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetProject = serde_json::from_slice(&msg).unwrap();

  let proj = get_data(data_path.clone(), submit.filename.clone());
  if proj.is_err() { error!("get_project proj"); return Err(proj.unwrap_err()); }
  let proj = proj.unwrap();
  let proj_nlist = serde_json::to_value(
    &to_nlist_proj(proj.clone())
  );
  if proj_nlist.is_err() { error!("get_project proj_nlist"); return Err(proj_nlist.unwrap_err().to_string()); }

  // Get corresponding template. 
  let template_filename = gen_filename(
    TEMPLATE_NAME.to_owned(), 
    proj["t_uuid"].as_str().unwrap().to_owned(), 
    Some(proj["t_ver"].as_u64().unwrap() as usize)
  );
  let temp = retrieve_decompress(
    get_savepath(data_path), template_filename);
  if temp.is_err() { error!("get_project temp"); return Err(temp.unwrap_err()); }
  let temp_nlist = serde_json::to_value(
    &to_nlist_temp(temp.unwrap()));
  if temp_nlist.is_err() { error!("get_project temp_nlist"); return Err(temp_nlist.unwrap_err().to_string()); }

  Ok(Some(json!({
    // "project": proj_nlist.unwrap(),
    "project": proj,  // we need to do some testing. 
    "template": temp_nlist.unwrap()
  }).to_string()))
}

// ===========================================
// POST and PUT
pub(crate) fn new_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitProject = serde_json::from_slice(&msg).unwrap();
  if submit.template_uuid.is_none() { 
    error!("new_project template_uuid");
    return Err("Template UUID cannot be null".to_owned()); 
  }

  let uuid = Uuid::now_v7().to_string();
  let filename = gen_filename(PROJECT_NAME.to_string(), uuid.clone(), None);

  let template_filename = gen_filename(
    TEMPLATE_NAME.to_owned(), 
    submit.template_uuid.clone().unwrap(), 
    None
  );
  let version = upd_ver_proj(
    get_verpath(data_path.clone()), template_filename, data_path.clone());
  if version.is_err() { error!("new_project get_verpath"); return Err(version.unwrap_err()); }

  let template_filename = gen_filename(
    TEMPLATE_NAME.to_owned(), 
    submit.template_uuid.clone().unwrap(), 
    Some(version.clone().unwrap())
  );
  let temp = retrieve_decompress(
    get_savepath(data_path.clone()), template_filename);
  if temp.is_err() { error!("new_project retrieve_decompress"); return Err(temp.unwrap_err()); }

  let data = submit.new_project(uuid.clone(), version.unwrap(), temp.unwrap());
  if data.is_err() { error!("new_project new_project"); return Err(data.unwrap_err()); }

  let data = data.unwrap();
  let data_path = modify_datapath(data_path.clone());
  let ret = compress_and_save(
    data.to_string(), data_path, filename.clone());
  if ret.is_err() { error!("new_project compress_and_save"); return Err(ret.unwrap_err()); }

  Ok(Some(json!({ "filename": filename }).to_string()))
}

pub(crate) fn edit_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitProject = serde_json::from_slice(&msg).unwrap();
  if submit.filename.is_none() { error!("edit_project filename"); 
    return Err("Filename cannot be null".to_owned()); }

  let old_serde = get_data(
    data_path.clone(), submit.filename.clone().unwrap());
  if old_serde.is_err() { error!("edit_project old_serde"); return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let edited_serde = submit.edit_project(old_serde).unwrap();  // no Err branch confirmed. 
  let ret = compress_and_save(edited_serde.to_string(), 
  modify_datapath(data_path.clone()), submit.filename.clone().unwrap());
  if ret.is_err() { error!("edit_project compress_and_save"); return Err(ret.unwrap_err()); }

  Ok(Some(edited_serde.to_string()))
}

// ================================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  let mut data_path = data_path;
  data_path.push("project");
  data_path
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}

// fn modify_datapath_temp(data_path: PathBuf) -> PathBuf {
//   let mut data_path = data_path;
//   data_path.push("template");
//   data_path
// }