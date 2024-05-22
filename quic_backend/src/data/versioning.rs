/// Template initially not save, only when project first save does it register to json. 
/// Then, we make a copy of the file to some place. (Not yet done)
/// When we update template after first project created, we change the boolean. 
/// Then, when a new project is added, we change the project version. 
/// If there are no change in template, nothing change. 

use crate::{messages::{CURR_VER_NULL, REQUEST_FAILED, UPD_VER_PROJ_FILE}, *};

use self::{compressor::{compress_and_save_fullpath, retrieve_decompress_fullpath}, file::{add_ver_json_zl, strip_ext}, messages::{CANNOT_FIND_VER, RD_CANNOT_FIND_FILE}};

/// Update Version Download Sample Template
pub(crate) fn upd_ver_sample(ver_path: PathBuf, key: String, filename: String) -> Result<Option<String>, String> {
  let index_url = "https://raw.githubusercontent.com/Wabinab/Triangulation_Sample/main/index.json";
  let resp = reqwest::blocking::get(index_url).unwrap();
  if !resp.status().is_success() { error!("download_sample failed reqwest index.json {:?}", resp); return Err(REQUEST_FAILED.to_owned()); }
  let index: Value = resp.json().unwrap();  // if can get, certainly will be json, no doubt.
  let curr_ver = index[key][2].clone();
  if curr_ver.is_null() { error!("Cannot find this file."); return Err(CURR_VER_NULL.to_owned()); }

  // Create file if not exist. Else, edit file. 
  let ver_file = retrieve_decompress_fullpath(ver_path.clone());
  if ver_file.clone().is_err_and(|x| x == RD_CANNOT_FIND_FILE.to_owned()) {
    // let output = File::create(ver_path.clone());
    // if output.is_err() { error!("upd_ver_sample ver_file create file"); return Err(UPD_VER_SAMPLE_FILE.to_owned()); }
    let res = compress_and_save_fullpath(json!({
      strip_ext(filename.clone()): curr_ver.clone()
    }).to_string(), ver_path.clone());
    if res.is_err() { error!("upd_ver_sample compress_and_save new"); return Err(res.unwrap_err()); }
    return Ok(None);
  }
  if ver_file.is_err() { error!("upd_ver_sample ver_file"); return Err(ver_file.unwrap_err()); }

  let mut ver_file = ver_file.unwrap();
  ver_file[strip_ext(filename.clone())] = json!(curr_ver.clone());
  let res = compress_and_save_fullpath(ver_file.to_string(), ver_path.clone());
  if res.is_err() { error!("upd_ver_sample compress_and_save existing"); return Err(res.unwrap_err()); }
  return Ok(None)
}

/// Update Version Save Project
/// temp_filename is (original non-versioned) template filename. 
/// We save a version of this template first before saving it version file, to ensure if it fails,
/// we don't need to revert changes. 
pub(crate) fn upd_ver_proj(ver_path: PathBuf, temp_filename: String, data_path: PathBuf) -> Result<Version, String> {
  let ver_file = retrieve_decompress_fullpath(ver_path.clone());
  if ver_file.clone().is_err_and(|x| x == RD_CANNOT_FIND_FILE.to_owned()) {
    let output = File::create(ver_path.clone());
    if output.is_err() { error!("upd_ver_proj ver_file create file"); return Err(UPD_VER_PROJ_FILE.to_owned()); }
    let save_ver = save_a_version_of_this_template(
      data_path, temp_filename.clone(), 0);
    if save_ver.is_err() { 
      error!("upd_ver_proj ver_file save_ver"); return Err(save_ver.unwrap_err()); }
    let res = compress_and_save_fullpath(json!({
      strip_ext(temp_filename.clone()): [0, UPDATE_VER]
    }).to_string(), ver_path.clone());
    if res.is_err() { error!("upd_ver_proj ver_file compress_and_save"); return Err(res.unwrap_err()); }
    return Ok(0);
  }
  if ver_file.is_err() { error!("upd_ver_proj ver_file"); return Err(ver_file.unwrap_err()); }

  let mut ver_file = ver_file.unwrap();
  let mut version: usize = 0;
  let mut updated = false;  // whether we updated version or not. 
  if ver_file[strip_ext(temp_filename.clone())].is_null() {
    ver_file[strip_ext(temp_filename.clone())] = json!([0, UPDATE_VER]);
    updated = true;
  } else {
    let mut data = ver_file[strip_ext(temp_filename.clone())].clone();
    if data[1] == !UPDATE_VER {   // Skip if otherwise. 
      version = (data[0].as_u64().unwrap() + 1) as usize;
      data[0] = json!(version.clone());
      data[1] = json!(UPDATE_VER);
      ver_file[strip_ext(temp_filename.clone())] = data;
      updated = true;
    }
  }
  if updated {
    let save_ver = save_a_version_of_this_template(
      data_path, temp_filename.clone(), version.clone());
    if save_ver.is_err() { error!("upd_ver_proj save_ver"); return Err(save_ver.unwrap_err()); }

    let res = compress_and_save_fullpath(ver_file.to_string(), ver_path);
    if res.is_err() { error!("upd_ver_proj compress_and_save"); return Err(res.unwrap_err()); }
  }
  Ok(version)
}

/// Update Version Save Template
pub(crate) fn upd_ver_temp(ver_path: PathBuf, filename: String) -> Result<(), String> {
  let ver_file = retrieve_decompress_fullpath(ver_path.clone());
  if ver_file.clone().is_err_and(|x| x == RD_CANNOT_FIND_FILE.to_owned()) {
    return Ok(());  // if cannot find file, skip. 
  }
  if ver_file.is_err() { error!("upd_ver_temp ver_file"); return Err(ver_file.unwrap_err()); }  // otherwise, return error message. 

  let mut ver_file = ver_file.unwrap();
  let mut data = ver_file[strip_ext(filename.clone())].clone();
  // If inexistent (no project created yet on this template) or nothing change (no new project since last updated template), we return empty.
  if data.is_null() { return Ok(()); }
  if data[1] != UPDATE_VER { return Ok(());}

  // Only when something change. 
  data[1] = json!(!UPDATE_VER);
  ver_file[strip_ext(filename.clone())] = data;
  let res = compress_and_save_fullpath(ver_file.to_string(), ver_path);
  if res.is_err() { error!("upd_ver_temp compress_and_save"); return Err(res.unwrap_err()); }
  Ok(())
}

/// Get the current version number of a template. 
pub(crate) fn get_ver(ver_path: PathBuf, filename: String) -> Result<Version, String> {
  let ver_file = retrieve_decompress_fullpath(ver_path.clone());
  if ver_file.is_err() { error!("get_ver ver_file"); return Err(ver_file.unwrap_err()); }
  let ver_file = ver_file.unwrap();

  let retval = ver_file[strip_ext(filename)].clone();
  if retval.is_null() { error!("get_ver retval is null"); return Err(CANNOT_FIND_VER.to_owned()); }
  Ok(retval[0].as_u64().unwrap() as usize)
}

/// Get versioning json file, given data path
pub(crate) fn get_verpath(data_path: PathBuf) -> PathBuf {
  let mut data_path = data_path;
  data_path.push("versioning.json.zl");
  data_path
}


// =============================================================================
/// Create a copy of template not yet done, if version updated. 
/// To be used in upd_ver_proj ONLY! 
fn save_a_version_of_this_template(data_path: PathBuf, filename: String, ver_no: usize) -> Result<(), String> {
  let mut from = modify_path(data_path.clone(), "template");
  from.push(filename.clone());
  let mut to = get_savepath(data_path);
  to.push(add_ver_json_zl(filename.clone(), ver_no));

  let res = fs::copy(from, to);
  if res.is_err() { error!("error saving a version of this template"); return Err(res.unwrap_err().to_string()); }
  Ok(())
}

// Get versioned template path. 
pub(crate) fn get_savepath(data_path: PathBuf) -> PathBuf {
  modify_path(data_path, "temp_versioned")
}

fn modify_path(data_path: PathBuf, extra: &'static str) -> PathBuf {
  let mut data_path = data_path;
  data_path.push(extra);
  data_path
}