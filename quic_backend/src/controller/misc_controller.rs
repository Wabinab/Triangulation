use crate::*;

use self::{file::gen_filename, misc_dto::SubmitGenFilename};

pub(crate) fn get_filename(msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGenFilename = serde_json::from_slice(&msg).unwrap();
  let name = if submit.type_name == "template" { TEMPLATE_NAME } else { PROJECT_NAME };
  let output = gen_filename(name.to_owned(), submit.uuid.clone(), None);
  Ok(Some(json!({
    "filename": output
  }).to_string()))
}


// pub(crate) fn get_bytes(data_path: PathBuf, msg: Bytes) -> Result<Option<Vec<u8>>, String> {
//   let submit: SubmitFilenameOnly = serde_json::from_slice(&msg).unwrap();
//   let path = modify_datapath(data_path, "template");
//   let fullpath = path.join(submit.filename.clone());
//   let contents = fs::read(fullpath.clone());
//   Ok(Some(contents.unwrap()))
// }