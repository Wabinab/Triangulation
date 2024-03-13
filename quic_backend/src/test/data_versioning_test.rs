use std::{fs::{self, File}, path::{Path, PathBuf}};

use serde_json::json;
use uuid::Uuid;

use crate::{compressor::{compress_and_save_fullpath, retrieve_decompress, retrieve_decompress_fullpath}, messages::CANNOT_FIND_VER, versioning::{get_ver, upd_ver_proj, upd_ver_temp}, UPDATE_VER};

fn cleanup(filepath: PathBuf) {
  fs::remove_file(filepath.as_path()).unwrap();
}

fn gen_testver_filename() -> String {
  let mut filename = Uuid::new_v4().to_string();
  filename.push_str("_test_ver.json.zl");
  return filename;
}

fn get_datapath() -> PathBuf {
  Path::new("../data").to_path_buf()
}

fn get_somename() -> String {
  "some-name.json.zl".to_string()
}

fn get_somename_stem() -> &'static str {
  "some-name"
}

// ============================================================
#[test]
fn test_versioning_nofile_works() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());
  assert_eq!(ret.ok(), Some(0));

  assert!(filepath.exists());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, UPDATE_VER]));

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
  assert!(data[get_somename_stem()].is_null());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok(), "{}", ret.unwrap_err());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, UPDATE_VER]));

  cleanup(filepath.to_path_buf());
}

#[test]
fn test_versioning_nochange() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());
  
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, UPDATE_VER]));

  cleanup(filepath.to_path_buf());
}

#[test]
fn test_versioning_next() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());

  // TBD upd_ver_temp
  let ret = upd_ver_temp(
    filepath.to_path_buf(), get_somename());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, !UPDATE_VER]));

  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());
  assert_eq!(ret.ok(), Some(1));
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([1, UPDATE_VER]));

  cleanup(filepath.to_path_buf());
}

#[test]
fn test_get_version_existing() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());

  let ver1 = get_ver(
    filepath.to_path_buf(), get_somename());
  assert!(ver1.is_ok_and(|x| x == 0));

  let new_filename = "filename.json.zl".to_string();
  assert_ne!(new_filename.clone(), get_somename());
  let ver2 = get_ver(
    filepath.to_path_buf(), new_filename.clone());
  assert!(ver2.is_err_and(|x| x == CANNOT_FIND_VER.to_owned()));

  cleanup(filepath.to_path_buf());
}

#[test]
fn test_temp_two_times_no_change() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());

  let ret = upd_ver_temp(
    filepath.to_path_buf(), get_somename());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, !UPDATE_VER]));

  // Call second time, no change. 
  let ret = upd_ver_temp(
    filepath.to_path_buf(), get_somename());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, !UPDATE_VER]));

  cleanup(filepath.to_path_buf())
}

#[test]
fn test_temp_no_file_not_raise() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_temp(
    filepath.to_path_buf(), get_somename());
  assert!(ret.is_ok());
  // No file creation, no need cleanup. 
}

#[test]
fn test_temp_is_null_not_raise() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_proj(
    filepath.to_path_buf(), get_somename(), get_datapath());
  assert!(ret.is_ok());

  let new_filename = "filename.json.zl".to_string();
  assert_ne!(new_filename.clone(), get_somename());
  let ret = upd_ver_temp(
    filepath.to_path_buf(), new_filename.clone());
  assert!(ret.is_ok());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[get_somename_stem()], json!([0, UPDATE_VER]));

  cleanup(filepath.to_path_buf())
}

