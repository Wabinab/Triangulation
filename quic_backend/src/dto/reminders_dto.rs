/// We might not use full name for each field later. 
/// The exact mappings will be done later. For now, just use full. 
use crate::*;
use crate::json::helper::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitPipeline {
  pub(crate) stage_step: u64,
  pub(crate) pipeline_id: u64,
  pub(crate) filename: String,
  pub(crate) locale: String
}

// =============================================================
/// Pipeline part only. 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Pipeline {
  pub(crate) id: u16,
  pub(crate) t: u8,  // card type
  pub(crate) title: String,
  pub(crate) questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Question {
  pub(crate) q: String, // question
  pub(crate) t: u8,  // question type
  // Answers?       // a
}
