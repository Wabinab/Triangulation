use crate::*;
use crate::obj_dto::Person;

use self::file::read_file;
use self::json::helper::find_by_id;
use self::reminders_dto::{val_to_stage, DemandPipeline, Stage};

pub(crate) fn echo(msg: Bytes) -> Result<Option<String>, String> {
    Ok(Some(String::from_utf8_lossy(&msg).to_string()))
}

pub(crate) fn _unused_eg_person(msg: Bytes) -> Result<Option<String>, String> {
    let _p: Person = serde_json::from_slice(&msg).unwrap();
    Ok(None)
}

// ===================================
pub(crate) fn get_sample_template(
    root: PathBuf, msg: Bytes
) -> Result<Option<String>, String> {
    let filename = String::from_utf8_lossy(&msg).to_string();
    let data = read_file(root.as_path(), filename);
    Ok(Some(String::from_utf8_lossy(&data).to_string()))
}


pub(crate) fn get_pipeline(
  root: PathBuf, msg: Bytes
) -> Result<Option<String>, String> {
  let p: DemandPipeline = serde_json::from_slice(&msg).unwrap();
  let filename = p.filename;
  let data = read_file(root.as_path(), filename);

  let sdata: Value = serde_json::from_slice(&data).unwrap();
  let stage = find_by_id(sdata["stages"].clone(), "step", p.stage_step);
  if stage.is_none() { return Err("Cannot find stages for this 'step'.".to_string()); }
  let b_stage = stage.unwrap();
  let pipeline = find_by_id(b_stage["pipeline"].clone(), "id", p.pipeline_id);
  if pipeline.is_none() { return Err("Cannot find pipeline for this 'id'.".to_string()); }

  Ok(Some(pipeline.unwrap().to_string()))
}