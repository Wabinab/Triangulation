use std::{fs::File, path::{Path, PathBuf}};

use serde_json::{json, Value};
use uuid::Uuid;

use crate::{compressor::{compress_and_save, compress_and_save_fullpath, retrieve_decompress_fullpath}, file::{add_ver_json_zl, strip_ext}, messages::CANNOT_FIND_VER, versioning::{get_savepath, get_ver, upd_ver_proj, upd_ver_temp}, UPDATE_VER};

use super::helper::{get_datapath, cleanup};

// fn cleanup(filepath: PathBuf) {
//   fs::remove_file(filepath.as_path()).unwrap();
// }

fn gen_testver_filename() -> String {
  let mut filename = Uuid::new_v4().to_string();
  filename.push_str("_test_ver.json.zl");
  return filename;
}

// fn get_datapath() -> PathBuf {
//   Path::new("../data").to_path_buf()
// }

fn get_tempfile_path(filename: String) -> PathBuf {
  let mut filepath = get_datapath();
  filepath.push("template");
  filepath.push(filename);
  filepath
}

// fn get_somename() -> String {
//   "some-name.json.zl".to_string()
// }

// fn get_somename_stem() -> &'static str {
//   "some-name"
// }

fn get_uuid_somename() -> String {
  let mut filename = Uuid::new_v4().to_string();
  filename.push_str("some-name.json.zl");
  filename
}

fn create_template_file() -> String {
  let filename = get_uuid_somename();
  let mut filepath = get_template_path();
  filepath.push(filename.clone());
  File::create(filepath).unwrap();
  return filename;
}

fn get_template_path() -> PathBuf {
  let mut filepath = get_datapath();
  filepath.push("template");
  filepath
}

// ============================================================
#[test]
fn test_versioning_nofile_works() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file();

  // Check file NOT exists
  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(check_file.as_path()).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());
  assert_eq!(ret.ok(), Some(0));

  assert!(filepath.exists());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, UPDATE_VER]));

  // Check file exists
  assert!(Path::new(check_file.as_path()).exists());

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(get_tempfile_path(temp_filename));
}

#[test]
fn test_versioning_exist_file_works() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  File::create(filepath).unwrap();
  let g = compress_and_save_fullpath(
    json!({}).to_string(), filepath.to_path_buf());
  assert!(g.is_ok(), "{}", g.unwrap_err());
  assert!(filepath.exists());

  let temp_filename = create_template_file();

  // Check file NOT exist
  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(&check_file).exists());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert!(data[strip_ext(temp_filename.clone())].is_null());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok(), "{}", ret.unwrap_err());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, UPDATE_VER]));

  // Check file exists
  assert!(Path::new(&check_file).exists());

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(get_tempfile_path(temp_filename));
}

#[test]
fn test_versioning_nochange() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file(); 

  // Check file NOT exists.
  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(&check_file).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());
  
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, UPDATE_VER]));

  // Check v0 exists, but not v1
  assert!(Path::new(&check_file).exists());
  let mut check_file_1 = get_savepath(get_datapath());
  check_file_1.push(add_ver_json_zl(temp_filename.clone(), 1));
  assert!(!Path::new(&check_file_1).exists());

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(get_tempfile_path(temp_filename));
}

#[test]
fn test_versioning_next() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file();

  // CHeck v0 not exist
  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(&check_file).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());

  let ret = upd_ver_temp(
    filepath.to_path_buf(), temp_filename.clone());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, !UPDATE_VER]));

  // Check v0 exists, but not v1
  assert!(Path::new(&check_file).exists());
  let mut check_file_1 = get_savepath(get_datapath());
  check_file_1.push(add_ver_json_zl(temp_filename.clone(), 1));
  assert!(!Path::new(&check_file_1).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());
  assert_eq!(ret.ok(), Some(1));
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([1, UPDATE_VER]));

  // Check both v0 and v1 exists. 
  // Check v0 exists, but not v1
  assert!(Path::new(&check_file).exists());
  assert!(Path::new(&check_file_1).exists());

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(check_file_1);
  cleanup(get_tempfile_path(temp_filename));
}

#[test]
fn test_get_version_existing() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file();

  // Check file not exist
  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(&check_file).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());

  let ver1 = get_ver(
    filepath.to_path_buf(), temp_filename.clone());
  assert!(ver1.is_ok_and(|x| x == 0));

  let new_filename = "filename.json.zl".to_string();
  assert_ne!(new_filename.clone(), temp_filename.clone());
  let ver2 = get_ver(
    filepath.to_path_buf(), new_filename.clone());
  assert!(ver2.is_err_and(|x| x == CANNOT_FIND_VER.to_owned()));

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(get_tempfile_path(temp_filename));
}

#[test]
fn test_temp_two_times_no_change() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file(); 

  // Check file not exist
  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(&check_file).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());

  let ret = upd_ver_temp(
    filepath.to_path_buf(), temp_filename.clone());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, !UPDATE_VER]));

  // Call second time, no change. 
  let ret = upd_ver_temp(
    filepath.to_path_buf(), temp_filename.clone());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, !UPDATE_VER]));

  assert!(Path::new(&check_file).exists());

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(get_tempfile_path(temp_filename));
}

#[test]
fn test_temp_no_file_not_raise() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());
  let ret = upd_ver_temp(
    filepath.to_path_buf(), "some-name.json.zl".to_owned());
  assert!(ret.is_ok());
  // No file creation, no need cleanup. 
}

#[test]
fn test_temp_is_null_not_raise() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file(); 

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());

  let new_filename = "filename.json.zl".to_string();
  assert_ne!(new_filename.clone(), temp_filename.clone());
  let ret = upd_ver_temp(
    filepath.to_path_buf(), new_filename.clone());
  assert!(ret.is_ok());

  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, UPDATE_VER]));

  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(Path::new(&check_file).exists());

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(get_tempfile_path(temp_filename));
}


#[test]
fn test_data_after_change_version_different() {
  let filename = gen_testver_filename();
  let filepath = Path::new(&filename);
  assert!(!filepath.exists());

  let temp_filename = create_template_file();

  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {"ty": 0, "title": "Title 1", "others": ["array/whatever as it is"]},
          {"ty": 0, "title": "Title 2", "others": ["array/whatever as it is"]}
        ]},
        {"name": "Stage 2", "pipeline": [

        ]}
    ]
  }"#;
  let old_serde: Value = serde_json::from_str(&c).unwrap();
  let ret = compress_and_save(old_serde.to_string(), 
    get_template_path(), temp_filename.to_owned());
  assert!(ret.is_ok());

  let mut check_file = get_savepath(get_datapath());
  check_file.push(add_ver_json_zl(temp_filename.clone(), 0));
  assert!(!Path::new(&check_file).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());

  let ret = upd_ver_temp(
    filepath.to_path_buf(), temp_filename.clone());
  assert!(ret.is_ok());
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([0, !UPDATE_VER]));

  let mut new_serde = old_serde.clone();
  new_serde["stages"][1]["name"] = json!("New Stage 2 Name");
  let ret = compress_and_save(new_serde.to_string(), 
  get_template_path(), temp_filename.clone());
  assert!(ret.is_ok());

  // Check v0 exists, but not v1
  assert!(Path::new(&check_file).exists());
  let mut check_file_1 = get_savepath(get_datapath());
  check_file_1.push(add_ver_json_zl(temp_filename.clone(), 1));
  assert!(!Path::new(&check_file_1).exists());

  let ret = upd_ver_proj(
    filepath.to_path_buf(), temp_filename.clone(), get_datapath());
  assert!(ret.is_ok());
  assert_eq!(ret.ok(), Some(1));
  let data = retrieve_decompress_fullpath(filepath.to_path_buf()).unwrap();
  assert_eq!(data[strip_ext(temp_filename.clone())], json!([1, UPDATE_VER]));

  // Check both v0 and v1 exists. 
  assert!(Path::new(&check_file).exists());
  assert!(Path::new(&check_file_1).exists());

  // Then check both file different
  let v0_serde = retrieve_decompress_fullpath(check_file.clone()).unwrap();
  let v1_serde = retrieve_decompress_fullpath(check_file_1.clone()).unwrap();
  assert_eq!(v0_serde, old_serde);
  assert_eq!(v1_serde, new_serde);
  assert_ne!(v0_serde, v1_serde);

  cleanup(filepath.to_path_buf());
  cleanup(check_file);
  cleanup(check_file_1);
  cleanup(get_tempfile_path(temp_filename));
}