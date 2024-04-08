use crate::file::{add_ver_json_zl, gen_filename, modify_datapath, strip_ext};

use super::helper::get_datapath;

#[test]
fn test_split_ext_1() {
  let filename = "some-name.json.zl".to_string();
  assert_eq!("some-name".to_owned(), strip_ext(filename));
}

#[test]
fn test_split_ext_2() {
  let filename = "some-name.meh".to_string();
  assert_eq!("some-name".to_owned(), strip_ext(filename));
}

#[test]
fn test_split_ext_3() {
  let filename = "some-name".to_string();
  assert_eq!("some-name".to_owned(), strip_ext(filename));
}

#[test]
fn test_gen_filename_with_version() {
  let name = "T";
  let uuid = "someuuid";
  let version = 7;
  let filename = gen_filename(name.to_owned(), uuid.to_owned(), Some(version));
  assert_eq!(filename, format!("{}_{}_V{}.json.zl", name, uuid, version));
}

#[test]
fn test_add_ver_json_zl() {
  let filename = "some-name.json.zl".to_string();
  let version = 5;
  assert_eq!("some-name_V5.json.zl".to_owned(), add_ver_json_zl(filename, version));
}

#[test]
fn test_add_ver_non_json_zl_become_json_zl() {
  let filename = "some-name.meh".to_string();
  let version = 5;
  assert_eq!("some-name_V5.json.zl".to_owned(), add_ver_json_zl(filename, version));
}

#[test]
fn test_modify_datapath() {
  let orig_datapath = get_datapath();
  let mut data_path = orig_datapath.clone();
  let new_datapath = modify_datapath(data_path.clone(), "template");

  data_path.push("template");
  assert_eq!(data_path, new_datapath);
  assert_ne!(orig_datapath, new_datapath);
}