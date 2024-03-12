use crate::file::strip_ext;

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