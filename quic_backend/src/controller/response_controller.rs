use crate::*;

use self::{compressor::{compress_and_save, retrieve_decompress}, pipeline_dto::{PipelineTrait, SubmitPipeline}, response_dto::{ResponseTrait, SubmitResponse}};

pub(crate) fn edit_response(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  modify_response(data_path, msg, CRUD::Update)
}

pub(crate) fn delete_response(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  modify_response(data_path, msg, CRUD::Delete)
}

pub(crate) fn get_response(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitPipeline = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("get_response old_serde"); return Err(old_serde.unwrap_err()); }
  let response = submit.get_response(old_serde.unwrap());
  if response.is_err() { error!("get_response response"); return Err(response.unwrap_err()); }
  Ok(Some(response.unwrap().to_string()))
}

// ===========================================
fn modify_response(data_path: PathBuf, msg: Bytes, crud: CRUD) -> Result<Option<String>, String> {
  let submit: SubmitResponse = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("modify_response old_serde"); return Err(old_serde.unwrap_err()); }
  // let t_uuid = old_serde.clone().unwrap()["t_uuid"].as_str().unwrap().to_owned();

  let edited_serde = match crud {
    // Because we don't use Create, we put it on the right, so it matches on first try.
    CRUD::Update | CRUD::Create => submit.edit_response(old_serde.unwrap()),
    CRUD::Delete => submit.delete_response(old_serde.unwrap())
  };
  if edited_serde.is_err() { error!("modify_response edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_response compress_and_save"); return Err(ret.unwrap_err()); }

  // Update versioning only done in new_project. 
  // // Update versioning if applicable. 
  // let template_filename = gen_filename(TEMPLATE_NAME.to_owned(), 
  //   t_uuid, None);
  // let ret2 = upd_ver_proj(get_verpath(data_path.clone()), 
  //   template_filename, data_path.clone());
  // if ret2.is_err() { error!("modify_response upd_ver_proj"); return Err(ret2.unwrap_err()); }

  Ok(Some(edited_serde.unwrap().to_string()))
}

// ===========================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  let mut data_path = data_path;
  data_path.push("project");
  data_path
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}