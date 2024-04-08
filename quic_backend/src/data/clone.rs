/// For cloning file. 
/// 
/// The uuid is changed when clone file. 
/// As for file name, we'll just add a (Copy) to the back.
/// If copy twice, still only (Copy) since we don't need unique. 
/// They can change their file name later. 

use std::path::PathBuf;

use log::error;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{compressor::{compress_and_save, retrieve_decompress}, file::{gen_filename, modify_datapath}, CloneType, PROJECT_NAME, TEMPLATE_NAME};

/// Clone Template
pub(crate) fn clone_template(data_path: PathBuf, filename: String) -> Result<Value, String> {
  clone(data_path, filename, CloneType::Template)
}


/// Clone project
pub(crate) fn clone_project(data_path: PathBuf, filename: String) -> Result<Value, String> {
  clone(data_path, filename, CloneType::Project)
}


fn clone(data_path: PathBuf, filename: String, clone_type: CloneType) -> Result<Value, String> {
  let data =  retrieve_decompress(data_path.clone(), filename);
  if data.is_err() { error!("clone_template retrieve_decompress"); return Err(data.unwrap_err()); }

  let mut new_data = data.unwrap();
  new_data["name"] = modify_name(new_data["name"].as_str().unwrap().to_owned());

  let uuid = Uuid::now_v7().to_string();
  new_data["uuid"] = json!(uuid.clone());
  let t_or_p_name = match clone_type {
    CloneType::Template => TEMPLATE_NAME.to_string(),
    CloneType::Project => PROJECT_NAME.to_string()
  };
  let new_filename = gen_filename(t_or_p_name, uuid.clone(), None);

  let t_or_p_path = match clone_type {
    CloneType::Template => "template",
    CloneType::Project => "project"
  };
  let data_path = modify_datapath(data_path, t_or_p_path);
  let ret = compress_and_save(
    new_data.to_string(), data_path, new_filename.clone());
  if ret.is_err() { error!("clone_template ret"); return Err(ret.unwrap_err()); }

  Ok(json!({
    "filename": new_filename.clone(),
    "data": new_data.clone()
  }))
}

// ========================================
fn modify_name(old_name: String) -> Value {
  let mut old_name = old_name;
  old_name.push_str(" (Copy)");
  json!(old_name)
}

// fn modify_datapath(data_path: PathBuf, path: &'static str) -> PathBuf {
//   let mut data_path = data_path;
//   data_path.push(path);
//   data_path
// }