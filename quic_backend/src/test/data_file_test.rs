use crate::file::{add_ver_json_zl, gen_filename, strip_ext};

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