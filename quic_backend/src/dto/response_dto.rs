/// Contains the answers to the questions. 
/// Team up with "project_dto".

use crate::{messages::{ANS_CNAME_NONE, CYCLE_AT_LEAST_ONE, CYCLE_IDX_CANNOT_NULL, CYCLE_NAME_NULL, LEN_PIPELINE_NOT_MATCH, OOB_CYCLE_IDX, OOB_PIPELINE_IDX, OOB_STAGE_IDX}, *};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitResponse {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: usize,
  pub(crate) cycle_index: Option<usize>,
  pub(crate) cycle_name: Option<String>,
  pub(crate) answer: Option<Value>
}

pub(crate) trait ResponseTrait {
  // Add new cycle
  fn add_new_cycle(&self, old_serde: Value) -> Result<Value, String>;
  // Edit cycle
  fn edit_cycle(&self, old_serde: Value) -> Result<Value, String>;
  // Delete existing cycle
  fn delete_cycle(&self, old_serde: Value) -> Result<Value, String>;
  // Clear cycle
  fn clear_cycle(&self, old_serde: Value) -> Result<Value, String>;
  // Edit response.
  fn edit_response(&self, old_serde: Value) -> Result<Value, String>;
  /// If you want to clear it out. 
  fn delete_response(&self, old_serde: Value) -> Result<Value, String>;
}

impl ResponseTrait for SubmitResponse {
  fn add_new_cycle(&self, old_serde: Value) -> Result<Value, String> {
    if self.cycle_name.is_none() || self.cycle_name.as_ref().is_some_and(|x| x == &"".to_owned()) { 
      error!("response_dto add_new_cycle cycle_name null"); return Err(CYCLE_NAME_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() {
      error!("response_dto add_cycle stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto add_cycle pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    let mut cycle = pipeline[0].clone();  // clone cycle 0 as template. 
    if cycle.is_null() {
      error!("response_dto add_cycle cycle BUG PLEASE FIX"); return Err(OOB_CYCLE_IDX.to_owned()); }

    cycle["name"] = json!(self.cycle_name.clone().unwrap());
    let mut c = cycle["data"].as_array().unwrap().clone();
    c = c.iter_mut()
      .map(|_| json!(""))
      .collect();
    cycle["data"] = json!(c);

    let mut pipeline_data = pipeline.as_array().unwrap().clone();
    pipeline_data.push(cycle);

    new_serde["pipelines"][self.stage_index][self.pipeline_index] = json!(pipeline_data);
    Ok(new_serde)
  }

  fn edit_cycle(&self, old_serde: Value) -> Result<Value, String> {
    if self.cycle_name.is_none() || self.cycle_name.as_ref().is_some_and(|x| x == &"".to_owned()) {
      error!("response_dto edit_cycle cycle_name null"); return Err(CYCLE_NAME_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() {
      error!("response_dto edit_cycle stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto edit_cycle pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    let cycle = pipeline[self.cycle_index.unwrap()].clone();
    if cycle.is_null() {
      error!("response_dto edit_cycle cycle"); return Err(OOB_CYCLE_IDX.to_owned()); }

    new_serde["pipelines"][self.stage_index][self.pipeline_index]
      [self.cycle_index.unwrap()]["name"] = json!(self.cycle_name.clone().unwrap());
    Ok(new_serde)
  }

  fn delete_cycle(&self, old_serde: Value) -> Result<Value, String> {
    if self.cycle_index.is_none() {
      error!("response_dto delete_cycle cycle index null"); return Err(CYCLE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() {
      error!("response_dto delete_cycle stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto delete_cycle pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    
    let mut pipelines = pipeline.as_array().unwrap().clone();
    if self.cycle_index.unwrap() >= pipelines.len() {
      error!("response_dto delete_cycle cycle_index exceed pipeline."); 
      return Err(OOB_CYCLE_IDX.to_owned()); }
    if pipelines.len() == 1 {
      error!("response_dto delete_cycle pipelines only one left cannot delete.");
      return Err(CYCLE_AT_LEAST_ONE.to_owned()); }
    pipelines.remove(self.cycle_index.unwrap());  // returns removed item, if needed.

    new_serde["pipelines"][self.stage_index][self.pipeline_index] = json!(pipelines);
    Ok(new_serde)
  }

  fn clear_cycle(&self, old_serde: Value) -> Result<Value, String> {
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() {
      error!("response_dto clear_cycle stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto clear_cycle pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    
    // let pipelines = pipeline.as_array().unwrap().clone();
    let mut new_pipeline: Vec<Value> = Vec::new();
    let mut cycle = pipeline[0].clone();
    let mut c = cycle["data"].as_array().unwrap().clone();
    c = c.iter_mut()
      .map(|_| json!(""))
      .collect();
    cycle["data"] = json!(c);
    new_pipeline.push(cycle);

    new_serde["pipelines"][self.stage_index][self.pipeline_index] = json!(new_pipeline);
    Ok(new_serde)
  }

  /// Caveat with this? We don't check if answer match within range of the question. 
  /// So that's a fragility introduced. 
  /// Second, we also don't check for its type correct or not. 
  /// So that's another fragility introduced. 
  fn edit_response(&self, old_serde: Value) -> Result<Value, String> {
    // if self.answer.is_none() { error!("response_dto edit answer null"); return Err(ANSWER_NULL.to_owned()); }
    if self.answer.is_none() && self.cycle_name.is_none() {
      error!("response_dto answer cycle_name both none"); return Err(ANS_CNAME_NONE.to_owned()); }
    if self.cycle_index.is_none() { error!("response_dto edit cycle index null"); return Err(CYCLE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() { 
      error!("response_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    let cycles = pipeline[self.cycle_index.unwrap()].clone();
    if cycles.is_null() {
      error!("response_dto edit cycle"); return Err(OOB_CYCLE_IDX.to_owned()); }

    if self.cycle_name.is_some() {
      new_serde["pipelines"][self.stage_index][self.pipeline_index]
        [self.cycle_index.unwrap()]["name"] = json!(self.cycle_name.clone().unwrap());
    }

    if self.answer.is_some() {
      // Alternatively, check template pipeline length match answer. 
      // This ensures no change in length; that's why we first assign empty string
      // to answers. 
      if cycles["data"].as_array().unwrap().len() != self.answer.as_ref().unwrap().as_array().unwrap().len() {
        error!("response_dto edit length"); 
        return Err(LEN_PIPELINE_NOT_MATCH.to_owned());
      }
      new_serde["pipelines"][self.stage_index][self.pipeline_index]
        [self.cycle_index.unwrap()]["data"] = self.answer.clone().unwrap();
    }
    Ok(new_serde)
  }

  fn delete_response(&self, old_serde: Value) -> Result<Value, String> {
    if self.cycle_index.is_none() { error!("response_dto edit cycle index null"); return Err(CYCLE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() { 
      error!("response_dto delete stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("response_dto delete pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    let cycles = pipeline[self.cycle_index.unwrap()].clone();
    if cycles.is_null() {
      error!("response_dto delete cycle"); return Err(OOB_CYCLE_IDX.to_owned()); }
    
    // Name shouldn't change even if delete response. 
    // new_serde["pipelines"][self.stage_index][self.pipeline_index]
    //   [self.cycle_index.unwrap()]["cycle"] = json!("");
    let mut c = cycles["data"].as_array().unwrap().clone();
    c = c.iter_mut()
      .map(|_| json!(""))
      .collect();
    new_serde["pipelines"][self.stage_index][self.pipeline_index]
      [self.cycle_index.unwrap()]["data"] = json!(c);

    Ok(new_serde)
  }
}