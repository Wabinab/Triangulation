use crate::*;

use self::{checklist_dto::SubmitRespChecklist, compressor::{compress_and_save, retrieve_decompress}, kelly_dto::SubmitKelly, pipeline_dto::{PipelineTrait, SubmitPipeline}, response_dto::{ResponseTrait, SubmitResponse}};

pub(crate) fn edit_response(data_path: PathBuf, msg: Bytes, ty: CardTypes) -> Result<Option<String>, String> {
  // modify_response(data_path, msg, CRUD::Update)
  choose_ty(data_path, msg, ty, CRUD::Update)
}

pub(crate) fn delete_response(data_path: PathBuf, msg: Bytes, ty: CardTypes) -> Result<Option<String>, String> {
  choose_ty(data_path, msg, ty, CRUD::Delete)
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
 
  let edited_serde = match crud {
    // Because we don't use Create, we put it on the right, so it matches on first try.
    CRUD::Update | CRUD::Create => submit.edit_response(old_serde.unwrap()),
    CRUD::Delete => submit.delete_response(old_serde.unwrap()),
    _ => Err("modify_response Not Applicable".to_owned())
  };
  if edited_serde.is_err() { error!("modify_response edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_response compress_and_save"); return Err(ret.unwrap_err()); }

  // Update versioning only done in new_project. 

  Ok(Some(edited_serde.unwrap().to_string()))
}

fn modify_kelly_resp(data_path: PathBuf, msg: Bytes, crud: CRUD) -> Result<Option<String>, String> {
  let submit: SubmitKelly = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("modify_kelly_resp old_serde"); return Err(old_serde.unwrap_err()); }
 
  let edited_serde = match crud {
    // Because we don't use Create, we put it on the right, so it matches on first try.
    CRUD::Update | CRUD::Create => submit.edit_response(old_serde.unwrap()),
    CRUD::Delete => submit.delete_response(old_serde.unwrap()),
    _ => Err("modify_kelly_resp Not Applicable".to_owned())
  };
  if edited_serde.is_err() { error!("modify_kelly_resp edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_kelly_resp compress_and_save"); return Err(ret.unwrap_err()); }

  // Update versioning only done in new_project. 

  Ok(Some(edited_serde.unwrap().to_string()))
}

fn modify_checklist_resp(data_path: PathBuf, msg: Bytes, crud: CRUD) -> Result<Option<String>, String> {
  let submit: SubmitRespChecklist = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("modify_checklist_resp old_serde"); return Err(old_serde.unwrap_err()); }
 
  let edited_serde = match crud {
    // Because we don't use Create, we put it on the right, so it matches on first try.
    CRUD::Update | CRUD::Create => submit.edit_response(old_serde.unwrap()),
    CRUD::Delete => submit.delete_response(old_serde.unwrap()),
    _ => Err("modify_checklist_resp Not Applicable".to_owned())
  };
  if edited_serde.is_err() { error!("modify_checklist_resp edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_checklist_resp compress_and_save"); return Err(ret.unwrap_err()); }

  // Update versioning only done in new_project. 

  Ok(Some(edited_serde.unwrap().to_string()))
}

// ===========================================
fn choose_ty(data_path: PathBuf, msg: Bytes, ty: CardTypes, crud: CRUD) -> Result<Option<String>, String> {
  match ty {
    CardTypes::Reminder => modify_response(data_path, msg, crud),
    CardTypes::Kelly => modify_kelly_resp(data_path, msg, crud),
    CardTypes::Checklist => modify_checklist_resp(data_path, msg, crud),
    _ => {
      error!("resp_controller: choose_ty: 'ty' string don't match any.");
      return Err(format!("{:?} Response: None of the ty matches.", crud));
    }
  }
}

fn modify_datapath(data_path: PathBuf) -> PathBuf {
  file::modify_datapath(data_path, "project")
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}