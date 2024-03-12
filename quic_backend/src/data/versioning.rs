use crate::*;

use self::{compressor::{compress_and_save_fullpath, retrieve_decompress_fullpath}, file::strip_ext};

/// Update Version Save Project
pub(crate) fn upd_ver_proj(ver_path: PathBuf, filename: String) -> Result<(), String> {
  let ver_file = retrieve_decompress_fullpath(ver_path.clone());
  if ver_file.clone().is_err_and(|x| x == "retrieve_decompress: cannot find file.".to_owned()) {
    let output = File::create(ver_path.clone());
    if output.is_err() { return Err("upd_ver_proj failed create file.".to_owned()); }
    let res = compress_and_save_fullpath(json!({
      strip_ext(filename.clone()): [0, UPDATE_VER]
    }).to_string(), ver_path.clone());
    if res.is_err() { return Err(res.unwrap_err()); }
    return Ok(());
  }
  if ver_file.is_err() { return Err(ver_file.unwrap_err()); }

  let mut ver_file = ver_file.unwrap();
  if ver_file[strip_ext(filename.clone())].is_null() {
    ver_file[strip_ext(filename.clone())] = json!([0, UPDATE_VER])
  } else {
    let mut data = ver_file[strip_ext(filename.clone())].clone();
    if (data[1] == !UPDATE_VER) {
      data[0] = json!(data[0].as_u64().unwrap() + 1);
      data[1] = json!(UPDATE_VER);
      ver_file[strip_ext(filename.clone())] = data;
    }
  }
  let res = compress_and_save_fullpath(ver_file.to_string(), ver_path);
  if res.is_err() { return Err(res.unwrap_err()); }
  Ok(())
}


pub(crate) fn get_ver(ver_path: PathBuf, filename: String) -> Result<Version, String> {
  let ver_file = retrieve_decompress_fullpath(ver_path.clone());
  if ver_file.is_err() { return Err(ver_file.unwrap_err()); }
  let ver_file = ver_file.unwrap();

  let retval = ver_file[strip_ext(filename)].clone();
  if retval.is_null() { return Err("Cannot find version for this filename.".to_owned()); }
  Ok(retval[0].as_u64().unwrap() as usize)
}