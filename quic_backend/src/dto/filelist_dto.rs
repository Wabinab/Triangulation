use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitFileList {
  pub(crate) page_no: usize,
  pub(crate) page_size: usize
}