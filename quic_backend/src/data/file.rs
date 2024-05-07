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
