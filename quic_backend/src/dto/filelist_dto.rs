use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitFileList {
  pub(crate) page_no: usize,
  pub(crate) page_size: usize
}

// #[derive(Serialize, Deserialize, Debug)]
// pub(crate) struct ReturnFileList {
//   // page_no and page_size no need?
//   pub(crate) total_count: usize,
//   pub(crate) data: Value
// }