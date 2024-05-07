// use std::ffi::OsString;

/// This controls the sample template

use crate::*;

use self::{compressor::retrieve_decompress, pipeline_dto::{PipelineTrait, SubmitPipeline}, template_dto::{to_nlist_temp, SubmitGetTemplate}};

pub(crate) fn get_downloaded_list(data_path: PathBuf) -> Result<Option<String>, String> {
  info!("Enter here");
  info!("{:?}", modify_datapath(data_path.clone()));
  let paths = fs::read_dir(modify_datapath(data_path)).unwrap();
  let filenames: Vec<String> = paths
    .map(|c| c.unwrap().file_name().into_string().unwrap())
    .collect();
  // let filenames2: Vec<String> = filenames.into_iter().map(|c| c.into_string()).collect();
  Ok(Some(json!({"data": filenames}).to_string()))
}

pub(crate) fn get_sample_nlist(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetTemplate = serde_json::from_slice(&msg).unwrap();

  let data = get_data(data_path, submit.filename.clone());
  if data.is_err() { error!("get_sample_nlist data"); return Err(data.unwrap_err()); }
  let retval = serde_json::to_string(
    &to_nlist_temp(data.unwrap()));
  if retval.is_err() { error!("get_sample_nlist retval"); return Err(retval.unwrap_err().to_string()); }

  Ok(Some(retval.unwrap()))
}

pub(crate) fn get_sample_pipeline(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitPipeline = serde_json::from_slice(&msg).unwrap();
  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("get_pipeline old_serde"); return Err(old_serde.unwrap_err()); }
  let pipeline = submit.get_pipeline(old_serde.unwrap());
  if pipeline.is_err() { error!("get_pipeline pipeline"); return Err(pipeline.unwrap_err()); }
  Ok(Some(pipeline.unwrap().to_string()))
}


// ==============================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  file::modify_datapath(data_path, "sample_templ")
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}