use crate::*;
use self::{compressor::{compress_and_save, retrieve_decompress}, kelly_dto::SubmitKelly, pipeline_dto::{PipelineTrait, PipelineViaProjTrait, SubmitPipeline, SubmitPipelineViaProj}, reminder_dto::{ReminderTrait, SubmitReminder}, versioning::{get_savepath, get_verpath, upd_ver_temp}};

pub(crate) fn new_pipeline(data_path: PathBuf, msg: Bytes, ty: usize) -> Result<Option<String>, String> {
  choose_ty(data_path, msg, ty, CRUD::Create)
}

pub(crate) fn edit_pipeline(data_path: PathBuf, msg: Bytes, ty: usize) -> Result<Option<String>, String> {
  choose_ty(data_path, msg, ty, CRUD::Update)
}

pub(crate) fn delete_pipeline(data_path: PathBuf, msg: Bytes, ty: usize) -> Result<Option<String>, String> {
  choose_ty(data_path, msg, ty, CRUD::Delete)
}

pub(crate) fn get_pipeline(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitPipeline = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("get_pipeline old_serde"); return Err(old_serde.unwrap_err()); }
  let pipeline = submit.get_pipeline(old_serde.unwrap());
  if pipeline.is_err() { error!("get_pipeline pipeline"); return Err(pipeline.unwrap_err()); }
  Ok(Some(pipeline.unwrap().to_string()))
}

pub(crate) fn get_pipeline_by_uuid_ver(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitPipelineViaProj = serde_json::from_slice(&msg).unwrap();
  let filename = submit.get_filename();
  let old_serde = retrieve_decompress(get_savepath(data_path.clone()), filename);
  if old_serde.is_err() { error!("get_pipeline_by_uuid_ver old_serde"); return Err(old_serde.unwrap_err()); }
  let pipeline = submit.get_pipeline(old_serde.unwrap());
  if pipeline.is_err() { error!("get_pipeline_by_uuid_ver pipeline"); return Err(pipeline.unwrap_err()); }
  Ok(Some(pipeline.unwrap().to_string()))
}

// =====================================================================================
/// Either new_reminder or edit_reminder, use this. 
fn modify_reminder(data_path: PathBuf, msg: Bytes, crud: CRUD) -> Result<Option<String>, String> {
  let submit: SubmitReminder = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("modify_reminder old_serde"); return Err(old_serde.unwrap_err()); }

  let edited_serde = match crud {
    CRUD::Create => submit.new_reminder(old_serde.unwrap()),
    CRUD::Update => submit.edit_reminder(old_serde.unwrap()),
    CRUD::Delete => submit.delete_reminder(old_serde.unwrap())
  };
  if edited_serde.is_err() { error!("modify_reminder edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_reminder compress_and_save"); return Err(ret.unwrap_err()); }

  // Update versioning if applicable. 
  let ret = upd_ver_temp(get_verpath(data_path.clone()),
    submit.filename.clone());
  if ret.is_err() { error!("modify_reminder upd_ver_temp"); return Err(ret.unwrap_err()); }

  Ok(Some(edited_serde.unwrap().to_string()))
}


/// We clone this from modify_reminder, because implement dynamic type T is too difficult
/// Plus one can't be sure one'll certainly always call such functions in the future. 
fn modify_kelly(data_path: PathBuf, msg: Bytes, crud: CRUD) -> Result<Option<String>, String> {
  let submit: SubmitKelly = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("modify_kelly old_serde"); return Err(old_serde.unwrap_err()); }

  let edited_serde = match crud {
    CRUD::Create => submit.new_reminder(old_serde.unwrap()),
    CRUD::Update => submit.edit_reminder(old_serde.unwrap()),
    CRUD::Delete => submit.delete_reminder(old_serde.unwrap())
  };
  if edited_serde.is_err() { error!("modify_kelly edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("modify_kelly compress_and_save"); return Err(ret.unwrap_err()); }

  // Update versioning if applicable. 
  let ret = upd_ver_temp(get_verpath(data_path.clone()),
    submit.filename.clone());
  if ret.is_err() { error!("modify_kelly upd_ver_temp"); return Err(ret.unwrap_err()); }

  Ok(Some(edited_serde.unwrap().to_string()))
}


// ===================================================
fn choose_ty(data_path: PathBuf, msg: Bytes, ty: usize, crud: CRUD) -> Result<Option<String>, String> {
  match ty {
    0 => modify_reminder(data_path, msg, crud),
    1 => modify_kelly(data_path, msg, crud),
    _ => {
      error!("pipeline_controller choose_ty No ty matches.");
      Err(format!("{:?} Pipeline: None of the ty matches.", crud))
    }
  }
}

fn modify_datapath(data_path: PathBuf) -> PathBuf {
  let mut data_path = data_path;
  data_path.push("template");
  data_path
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}