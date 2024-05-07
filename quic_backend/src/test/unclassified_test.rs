use std::{ffi::OsString, fs};

#[test]
fn test_get_files() {
  let paths = fs::read_dir("../cert").unwrap();

  assert_eq!(paths.count(), 4);
}

#[test]
fn test_get_files_with_skip_take() {
  let paths = fs::read_dir("../cert").unwrap();
  // let mut filenames: Vec<OsString> = Vec::new();

  // for path in paths.skip(2).take(2) {
  //   // println!("{:?}", path.unwrap().file_name());
  //   filenames.push(path.unwrap().file_name());
  // }
  let filenames: Vec<OsString> = paths.skip(2).take(2).into_iter()
    .map(|c| c.unwrap().file_name())
    .collect();
  assert_eq!(filenames.len(), 2);
  let mut paths = fs::read_dir("../cert").unwrap();
  assert_eq!(filenames[0], paths.nth(2).unwrap().unwrap().file_name());
  // Because nth will "pop" the value out, so we need to fetch again. 
  let mut paths = fs::read_dir("../cert").unwrap();
  assert_eq!(filenames[1], paths.nth(3).unwrap().unwrap().file_name());
  // assert_eq!(1, 2);
}