use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitGenFilename {
  pub(crate) type_name: String,
  pub(crate) uuid: String
}