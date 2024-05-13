// use std::ffi::OsString;

use std::io;

use uuid::Uuid;

/// This controls the sample template

use crate::{messages::{FAILED_COPY_CONTENT, FAILED_CREATE_FILE, REQUEST_FAILED, SUCCESS_DOWNLOAD}, *};

use self::{compressor::{compress_and_save_fullpath, retrieve_decompress, retrieve_decompress_fullpath}, file::gen_filename, misc_dto::SubmitFilenameOnly, pipeline_dto::{PipelineTrait, SubmitPipeline}, template_dto::{to_nlist_temp, SubmitGetTemplate}};

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

pub(crate) fn clone_sample_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitFilenameOnly = serde_json::from_slice(&msg).unwrap();
  let from_folder = modify_datapath(data_path.clone());
  let from = from_folder.join(submit.filename.clone());
  let to_folder = file::modify_datapath(data_path.clone(), "template");
  let uuid = Uuid::now_v7().to_string();
  let new_filename = gen_filename(TEMPLATE_NAME.to_string(), uuid.clone(), None);
  let to = to_folder.join(new_filename.clone());
  let res = fs::copy(from, to.clone());
  if res.is_err() { error!("clone_sample_template copy error."); return Err(res.unwrap_err().to_string()); }

  // Make change to the existing "uuid"
  let res_data = retrieve_decompress_fullpath(to.clone());
  if res_data.is_err() { error!("clone_sample_template retrieve error."); return Err(res_data.unwrap_err()); }
  let mut data = res_data.unwrap();
  data["uuid"] = json!(uuid.clone());
  let res2 = compress_and_save_fullpath(data.to_string(), to.clone());
  if res2.is_err() { error!("clone_sample_template save back error."); return Err(res2.unwrap_err()); }

  Ok(Some(json!({ "filename": new_filename }).to_string()))
}

pub(crate) fn download_sample_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let mut url: String = "https://github.com/Wabinab/Triangulation_Sample/raw/main/".to_owned();
  let submit: SubmitFilenameOnly = serde_json::from_slice(&msg).unwrap();
  url.push_str(&submit.filename.clone());

  let mut resp = reqwest::blocking::get(url).unwrap();
  if !resp.status().is_success() { error!("download_sample failed resp. {:?}", resp); return Err(REQUEST_FAILED.to_owned()); }
  
  let to_folder = modify_datapath(data_path.clone());
  let to = to_folder.join(submit.filename.clone());
  let out = File::create(to);
  if out.is_err() { error!("download_sample failed create file. {:?}", out.unwrap_err()); return Err(FAILED_CREATE_FILE.to_owned()); }
  let mut out = out.unwrap();

  let finale = io::copy(&mut resp, &mut out);
  if finale.is_err() { error!("download_sample io copy error. {:?}", finale.unwrap_err()); return Err(FAILED_COPY_CONTENT.to_owned()); }
  
  Ok(Some(json!({
    "msg": SUCCESS_DOWNLOAD.to_owned(),
    "total_bytes": finale.unwrap()
  }).to_string()))
}

// ==============================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  file::modify_datapath(data_path, "sample_templ")
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}