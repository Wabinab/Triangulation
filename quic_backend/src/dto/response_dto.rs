/// Contains the answers to the questions. 
/// Team up with "project_dto".

use crate::{messages::{ANSWER_NULL, LEN_PIPELINE_NOT_MATCH, OOB_PIPELINE_IDX, OOB_STAGE_IDX}, *};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitResponse {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: usize,
  pub(crate) answer: Option<Value>
}

pub(crate) trait ResponseTrait {
  fn edit_response(&self, old_serde: Value) -> Result<Value, String>;
  /// If you want to clear it out. 
  fn delete_response(&self, old_serde: Value) -> Result<Value, String>;
}

impl ResponseTrait for SubmitResponse {
  /// Caveat with this? We don't check if answer match within range of the question. 
  /// So that's a fragility introduced. 
  /// Second, we also don't check for its type correct or not. 
  /// So that's another fragility introduced. 
  fn edit_response(&self, old_serde: Value) -> Result<Value, String> {
    if self.answer.is_none() { error!("response_dto edit answer null"); return Err(ANSWER_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();
    if new_serde["pipelines"][self.stage_index].is_null() { 
      error!("response_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned());
    }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned());
    }
    if pipeline.as_array().unwrap().len() != self.answer.as_ref().unwrap().as_array().unwrap().len() {
      error!("response_dto edit length"); 
      // Alternatively, check template pipeline length match answer. 
      // This ensures no change in length; that's why we first assign empty string
      // to answers. 
      return Err(LEN_PIPELINE_NOT_MATCH.to_owned());
    }
    new_serde["pipelines"][self.stage_index][self.pipeline_index] = self.answer.as_ref().unwrap().clone();
    Ok(new_serde)
  }

  fn delete_response(&self, old_serde: Value) -> Result<Value, String> {
    let mut new_serde = old_serde.clone();
    if new_serde["pipelines"][self.stage_index].is_null() { 
      error!("response_dto delete stages"); return Err(OOB_STAGE_IDX.to_owned());
    }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned());
    }
    let mut pipelines = pipeline.as_array().unwrap().clone();
    pipelines = pipelines.iter_mut()
      .map(|_| json!(""))
      .collect();
    new_serde["pipelines"][self.stage_index][self.pipeline_index] = json!(pipelines);

    Ok(new_serde)
  }
}