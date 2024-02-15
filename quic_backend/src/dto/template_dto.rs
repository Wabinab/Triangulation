use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitNewTemplate {
  pub(crate) name: String,
  pub(crate) description: String
}