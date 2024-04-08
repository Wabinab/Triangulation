use std::path::PathBuf;

/// derive filename given name and uuid. 
pub(crate) fn gen_filename(name: String, uuid: String, version: Option<usize>) -> String {
  let mut filename: String = name.chars()
    .filter(|c| c.is_alphanumeric())
    .take(50)
    .collect();
  if filename.len() == 0 { filename = "untitled".to_owned(); }
  filename.push_str("_");
  filename.push_str(&uuid);
  if version.is_some() {
    filename.push_str("_V");
    filename.push_str(version.unwrap().to_string().as_str());
  }
  filename.push_str(".json.zl");

  filename
}

/// Strip extension from filename. 
/// some-name.json -> some-name
/// some-name.json.zl -> some-name. 
/// some-name -> some-name. 
/// Cannot be null, since we're not using Option. 
pub(crate) fn strip_ext(filename: String) -> String {
  let g = filename.split(".").next();
  g.unwrap().to_string()
}

/// Only to be used with .json.zl file. It didn't detect original extension. 
/// It's use to add _V1 or any other version to the back of file. 
/// E.g. some-name.json.zl becomes some-name_V1.json.zl. 
/// Any other extension will be replaced with .json.zl. 
pub(crate) fn add_ver_json_zl(filename: String, version: usize) -> String {
  let mut stripped: String = strip_ext(filename);
  stripped.push_str("_V");
  stripped.push_str(version.to_string().as_str());
  stripped.push_str(".json.zl");
  return stripped;
}

/// Modify datapath. 
pub(crate) fn modify_datapath(data_path: PathBuf, path: &'static str) -> PathBuf {
  let mut data_path = data_path;
  data_path.push(path);
  data_path
}

// pub(crate) fn read_file(root: &Path, path: String) -> Vec<u8> {
//   let path = Path::new(&path);
//   let mut real_path = PathBuf::from(root);
//   let mut components = path.components();
//   match components.next() {
//     Some(path::Component::RootDir) => {}
//     _ => {
//         error!("path must be absolute");
//     }
//   }
//   for c in components {
//     match c {
//       path::Component::Normal(x) => {
//         real_path.push(x);
//       }
//       x => {
//         error!("illegal component in path: {:?}", x);
//       }
//     }
//   }
//   match fs::read(&real_path) {
//     Ok(data) =>  data,
//     Err(_) => {
//       error!("Cannot read file.");
//       error!("{:?}", real_path);
//       return "Cannot find file".to_owned().into_bytes();
//     }
//   }
// }

// /// Returns true if create successful, else returns false. 
// pub(crate) fn create_file(root: &Path, path: &Path) -> Result<File, Error> {
//   // let path = Path::new(&path);
//   let mut real_path = PathBuf::from(root);
//   let mut components = path.components();
//   match components.next() {
//     Some(path::Component::RootDir) => {}
//     _ => {
//         error!("path must be absolute");
//     }
//   }
//   for c in components {
//     match c {
//       path::Component::Normal(x) => {
//         real_path.push(x);
//       }
//       x => {
//         error!("illegal component in path: {:?}", x);
//       }
//     }
//   }
//   File::create(&real_path)
// }

