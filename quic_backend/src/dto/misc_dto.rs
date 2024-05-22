use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitGenFilename {
  pub(crate) type_name: String,
  pub(crate) uuid: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitFilenameOnly {
  pub(crate) filename: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitDownload {
  pub(crate) filename: String,
  pub(crate) keyname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitUuidOnly {
  pub(crate) uuid: String
}