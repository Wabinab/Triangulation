use std::fs;

#[test]
fn test_get_files() {
  let paths = fs::read_dir("../cert").unwrap();

  assert_eq!(paths.count(), 4);
}