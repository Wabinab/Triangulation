use uuid::Uuid;

use crate::*;

use self::{file::{create_file, gen_filename}, template_dto::SubmitNewTemplate};

pub(crate) fn new_template(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitNewTemplate = serde_json::from_slice(&msg).unwrap();
  let mut filename = gen_filename(submit.name.clone());
  if filename.len() == 0 { filename = "untitled".to_owned(); }
  let path = Path::new("template");

  let uuid = Uuid::now_v7().to_string();
  filename.push_str("_");
  filename.push_str(&uuid);
  filename.push_str(".json.zl");
  let file = create_file(&data_path, path.join(filename).as_path());
  if file.is_err() { return Err(file.unwrap_err().to_string()); }

  
  Ok(Some("".to_owned()))
}