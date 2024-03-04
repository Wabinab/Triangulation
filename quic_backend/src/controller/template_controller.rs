use uuid::Uuid;

use crate::*;

use self::{compressor::{compress_and_save, retrieve_decompress}, file::{create_file, gen_filename}, reminders_dto::{SubmitPipeline, SubmitReminder, SubmitReminderTrait}, stage_dto::{SubmitEditStage, SubmitStageTrait}, template_dto::{SubmitEditTemplate, SubmitGetTemplate, SubmitTemplateTrait}
};

// =================================================
// GET
pub(crate) fn get_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();

  let mut data_path = data_path;
  data_path.push("template");
  let data = retrieve_decompress(data_path, submit.filename);
  if data.is_err() { return Err(data.unwrap_err()); }

  Ok(Some(data.unwrap().to_string()))
}


// =============================================
// POST and PUT
pub(crate) fn new_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitEditTemplate = serde_json::from_slice(&msg).unwrap();
  
  let uuid = Uuid::now_v7().to_string();
  let data = submit.to_new_serde(uuid.clone(), json!([]));

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
  // let path = Path::new("/template").join(filename.clone());
  // let file = create_file(&data_path, path.as_path());
  // if file.is_err() { return Err(file.unwrap_err().to_string()); }
  // let res = file.unwrap().write_all(data.to_string().as_bytes());
  // if res.is_err() { return Err(res.unwrap_err().to_string()); }

  Ok(Some(filename))
}



pub(crate) fn edit_template_stagelevel(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let mut data_path = data_path;
  data_path.push("template");
  
  // Edit Template (Name and Description)
  let submit: SubmitEditTemplate = serde_json::from_slice(&msg).unwrap();

  let old_serde = retrieve_decompress(data_path.clone(), submit.filename.clone().unwrap());
  if old_serde.is_err() { return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let edited_serde = submit.to_serde(old_serde);
  // Savings occur after we edit stage. 

  // Edit Stage
  let submit: SubmitEditStage = serde_json::from_slice(&msg).unwrap();
  
  let new_serde = submit.to_serde(edited_serde);
  let ret = compress_and_save(new_serde.to_string(), data_path.clone(), submit.filename.clone());
  if ret.is_err() { return Err(ret.unwrap_err()); }

  // We'll update to change filename too in the future. That isn't too important for now. 
  Ok(Some(new_serde.to_string()))
}


pub(crate) fn save_reminder(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let mut data_path = data_path;
  data_path.push("template");

  let submit: SubmitReminder = serde_json::from_slice(&msg).unwrap();

  let old_serde = retrieve_decompress(data_path.clone(), submit.filename.clone().unwrap());
  if old_serde.is_err() { return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let edited_serde = submit.to_serde(old_serde);
  if edited_serde.is_none() { return Err("There's an error with to_serde reminder_dto.".to_owned()); }
  let ret = compress_and_save(edited_serde.unwrap().to_string(), data_path.clone(), submit.filename.clone());
  if ret.is_err() { return Err(ret.unwrap_err()); }

  Ok(Some(edited_serde.unwrap().to_string()))
}