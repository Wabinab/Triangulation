use uuid::Uuid;

use crate::*;

use self::{compressor::{compress_and_save, retrieve_decompress}, 
  file::{create_file, gen_filename}, 
  stage_dto::{SubmitEditStage, SubmitStageTemplate, SubmitStageTrait}, 
  template_dto::{SubmitGetTemplate, SubmitNewTemplate, SubmitTemplateTrait}
};

pub(crate) fn new_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitNewTemplate = serde_json::from_slice(&msg).unwrap();
  
  let uuid = Uuid::now_v7().to_string();
  let data = submit.to_serde_new(uuid.clone(), json!([]));

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

pub(crate) fn get_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();

  let mut data_path = data_path;
  data_path.push("template");
  let data = retrieve_decompress(data_path, submit.filename);
  if data.is_err() { return Err(data.unwrap_err()); }

  Ok(Some(data.unwrap().to_string()))
}

pub(crate) fn edit_stages(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitEditStage = serde_json::from_slice(&msg).unwrap();

  let mut data_path = data_path;
  data_path.push("template");
  let old_serde = retrieve_decompress(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { return Err(old_serde.unwrap_err()); }
  let old_serde = data.unwrap();

  let new_serde = submit.to_serde(old_serde);
  let ret = compress_and_save(new_serde.to_string(), data_path.clone(), submit.filename.clone());
  if ret.is_err() { return Err(ret.unwrap_err()); }

  Ok(Some("Successfully edit stages.".to_owned()))
}


