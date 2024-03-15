use std::{fs, path::{Path, PathBuf}};

pub(crate) fn get_datapath() -> PathBuf {
  Path::new("../data").to_path_buf()
}

pub(crate) fn cleanup(filepath: PathBuf) {
  fs::remove_file(filepath.as_path()).unwrap();
}