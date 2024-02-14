use std::path::PathBuf;

use crate::*;

pub(crate) fn read_file(root: &Path, path: String) -> Vec<u8> {
  let path = Path::new(&path);
  let mut real_path = PathBuf::from(root);
  let mut components = path.components();
  match components.next() {
    Some(path::Component::RootDir) => {}
    _ => {
        error!("path must be absolute");
    }
  }
  for c in components {
    match c {
      path::Component::Normal(x) => {
        real_path.push(x);
      }
      x => {
        error!("illegal component in path: {:?}", x);
      }
    }
  }
  match fs::read(&real_path) {
    Ok(data) =>  data,
    Err(_) => {
      error!("Cannot read file.");
      error!("{:?}", real_path);
      return "Cannot find file".to_owned().into_bytes();
    }
  }
}