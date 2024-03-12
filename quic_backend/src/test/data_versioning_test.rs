use std::{fs::{self, File}, path::{Path, PathBuf}};

use serde_json::json;
use uuid::Uuid;

use crate::{compressor::{compress_and_save_fullpath, retrieve_decompress, retrieve_decompress_fullpath}, versioning::{get_ver, upd_ver_proj}, UPDATE_VER};

fn cleanup(filepath: PathBuf) {
  fs::remove_file(filepath.as_path()).unwrap();
}

fn gen_testver_filename() -> String {
  let mut filename = Uuid::new_v4().to_string();
  filename.push_str("_test_ver.json.zl");
  return filename;
}

// ============================================================
#[test]
fn test_versioning_nofile_works() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), "some-name.json.zl".to_string());
  assert!(ret.is_ok());

  assert!(filepath.exists());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data["some-name"], json!([0, UPDATE_VER]));

  cleanup(filepath.to_path_buf());
}

#[test]
fn test_versioning_exist_file_works() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  File::create(filepath).unwrap();
  let g = compress_and_save_fullpath(json!({
    // "cannot be empty": "yeah"
  }).to_string(), filepath.to_path_buf());
  assert!(g.is_ok(), "{}", g.unwrap_err());
  assert!(filepath.exists());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert!(data["some-name"].is_null());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), "some-name.json.zl".to_string());
  assert!(ret.is_ok(), "{}", ret.unwrap_err());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data["some-name"], json!([0, UPDATE_VER]));

  cleanup(filepath.to_path_buf());
}

#[test]
fn test_versioning_nochange() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), "some-name.json.zl".to_string());
  assert!(ret.is_ok());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), "some-name.json.zl".to_string());
  assert!(ret.is_ok());
  
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data["some-name"], json!([0, UPDATE_VER]));

  cleanup(filepath.to_path_buf());
}


#[test]
fn test_versioning_next() {
  // let filename = gen_testver_filename();
  // let filepath = Path::new(&filename);
  // assert!(!filepath.exists());
  // let ret = upd_ver_proj(
  //   filepath.to_path_buf(), "some-name.json.zl".to_string());
  // assert!(ret.is_ok());

  // // TBD upd_ver_temp
  // let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  // assert_eq!(data["some-name"], json!([0, !UPDATE_VER]));

  // let ret = upd_ver_proj(
  //   filepath.to_path_buf(), "some-name.json.zl".to_string());
  // assert!(ret.is_ok());
  // let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  // assert_eq!(data["some-name"], json!([1, UPDATE_VER]));

  // cleanup(filepath.to_path_buf());
}

#[test]
fn test_get_version_existing() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), "some-name.json.zl".to_string());
  assert!(ret.is_ok());

  let ver1 = get_ver(
    filepath.to_path_buf(), "some-name.json.zl".to_string());
  assert!(ver1.is_ok_and(|x| x == 0));

  let ver2 = get_ver(
    filepath.to_path_buf(), "filename.json.zl".to_string());
  assert!(ver2.is_err_and(|x| x == "Cannot find version for this filename.".to_owned()));

  cleanup(filepath.to_path_buf());
}