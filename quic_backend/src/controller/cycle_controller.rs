use crate::*;

use self::{compressor::{compress_and_save, retrieve_decompress}, file::modify_datapath, response_dto::{CycleTrait, SubmitResponse}};

// ==============================================
pub(crate) fn modify_cycle(data_path: PathBuf, msg: Bytes, crud: CRUD) -> Result<Option<String>, String> {
  let submit: SubmitResponse = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("modify_cycle old_serde"); return Err(old_serde.unwrap_err()); }

  let resp = match crud {
    CRUD::Create => submit.add_new_cycle(old_serde.unwrap()),
    CRUD::Update => submit.edit_cycle(old_serde.unwrap()),
    CRUD::Delete => submit.delete_cycle(old_serde.unwrap()),
    CRUD::Clear => submit.clear_cycle(old_serde.unwrap())
  };
  // let resp = submit.add_new_cycle(old_serde.unwrap());
  if resp.is_err() { error!("modify_cycle resp"); return Err(resp.unwrap_err()); }

  let ret = compress_and_save(resp.clone().unwrap().to_string(), 
    _modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_cycle compress_and_save"); return Err(ret.unwrap_err()); }

  Ok(Some(resp.unwrap().to_string()))
}

// ==============================================
fn _modify_datapath(data_path: PathBuf) -> PathBuf {
  modify_datapath(data_path, "project")
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = _modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}