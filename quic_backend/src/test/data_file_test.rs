use crate::file::{gen_filename, strip_ext};

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