use crate::{messages::{ANS_NONE, CHECKLIST_LEN_2, CHECKLIST_NONE, CHECKLIST_STRVEC, CL_EXTRA_LEN_NOT_MATCH, CYCLE_IDX_CANNOT_NULL, LEN_PIPELINE_NOT_MATCH, OOB_CYCLE_IDX, OOB_PIPELINE_IDX, OOB_STAGE_IDX, PIPELINE_IDX_CANNOT_NULL, TITLE_NONE, VEC_BOOL_ONLY}, *};

use self::{messages::NOT_IMPLEMENTED, reminder_dto::ReminderTrait, response_dto::ResponseTrait};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitChecklist {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: Option<usize>,
  pub(crate) title: Option<String>,
  pub(crate) checklist: Option<Vec<String>>  // share between compulsory and extra.
}

// Note: ty is always CHECKLIST_TYPE, so we don't check but force value equal. 
impl ReminderTrait for SubmitChecklist {
  fn new_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("checklist_dto title is none."); return Err(TITLE_NONE.to_owned()); }
    if self.checklist.is_none() { error!("checklist_dto checklist is none."); return Err(CHECKLIST_NONE.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("checklist_dto new stages"); return Err(OOB_STAGE_IDX.to_owned()); }

    let mut pipelines = stages["pipeline"].as_array().unwrap().clone();
    let data = json!({
      "ty": CHECKLIST_TYPE,
      "title": self.title.clone().unwrap(),
      "others": self.checklist.clone().unwrap()
    });
    pipelines.push(data);

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }

  fn edit_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("checklist_dto title is none."); return Err(TITLE_NONE.to_owned()); }
    if self.checklist.is_none() { error!("checklist_dto checklist is none."); return Err(CHECKLIST_NONE.to_owned()); }
    if self.pipeline_index.is_none() { error!("checklist_dto edit pipeline_index is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("checklist_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() { error!("checklist_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let data = json!({
      "ty": CHECKLIST_TYPE,
      "title": self.title.clone().unwrap(),
      "others": self.checklist.clone().unwrap()
    });

    new_serde["stages"][self.stage_index]["pipeline"][self.pipeline_index.unwrap()] = data;
    Ok(new_serde)
  }

  fn delete_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.pipeline_index.is_none() { error!("checklist_dto edit kelly_idx is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("checklist_dto delete stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() { error!("checklist_dto delete pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let bind_pipeline = stages["pipeline"].clone();
    let mut pipelines = bind_pipeline.as_array().unwrap().clone();
    pipelines.remove(self.pipeline_index.unwrap());

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }
}

// ==================================================================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitRespChecklist {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: usize,
  pub(crate) cycle_index: Option<usize>,
  pub(crate) checklist: Option<Value>,  
  pub(crate) extra_checklist: Option<Value>
}

impl ResponseTrait for SubmitRespChecklist {
  fn add_new_cycle(&self, _old_serde: Value) -> Result<Value, String> {
    Err(NOT_IMPLEMENTED.to_owned())
  }
  fn edit_cycle(&self, _old_serde: Value) -> Result<Value, String> {
    Err(NOT_IMPLEMENTED.to_owned())
  }
  fn delete_cycle(&self, _old_serde: Value) -> Result<Value, String> {
    Err(NOT_IMPLEMENTED.to_owned())
  }
  fn clear_cycle(&self, _old_serde: Value) -> Result<Value, String> {
    Err(NOT_IMPLEMENTED.to_owned())
  }

  fn edit_response(&self, old_serde: Value) -> Result<Value, String> {
    if self.checklist.is_none() {
      error!("checklist_dto answer none"); return Err(ANS_NONE.to_owned()); }
    if self.cycle_index.is_none() { error!("checklist_dto edit cycle index null"); return Err(CYCLE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() { 
      error!("checklist_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("checklist_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    let cycles = pipeline[self.cycle_index.unwrap()].clone();
    if cycles.is_null() {
      error!("checklist_dto edit cycle"); return Err(OOB_CYCLE_IDX.to_owned()); }
    if cycles["data"].as_array().unwrap().len() != self.checklist.as_ref().unwrap().as_array().unwrap().len() {
      error!("checklist_dto edit length"); 
      return Err(LEN_PIPELINE_NOT_MATCH.to_owned());
    }

    // Unlike usual data only, we have one "extra".
    if self.extra_checklist.is_some() {
      let checklist: Value = self.extra_checklist.clone().unwrap();
      let _c = checklist.as_array().unwrap().clone();
      if _c.len() != 2 { error!("checklist_dto _c checklist len != 2"); 
        return Err(CHECKLIST_LEN_2.to_owned()); }
      let _q: Result<Vec<String>, serde_json::Error> = serde_json::from_value(checklist[0].clone());
      if _q.is_err() { error!("checklist_dto _q question not pure string."); 
        return Err(CHECKLIST_STRVEC.to_owned()); }
      let _a: Result<Vec<bool>, serde_json::Error> = serde_json::from_value(checklist[1].clone());
      if _a.is_err() { error!("checklist_dto _a answer not pure bool."); 
        return Err(VEC_BOOL_ONLY.to_owned()); }
      if _q.unwrap().len() != _a.unwrap().len() { error!("checklist_dto question and answer array len different"); 
        return Err(CL_EXTRA_LEN_NOT_MATCH.to_owned()); }

      new_serde["pipelines"][self.stage_index][self.pipeline_index]
        [self.cycle_index.unwrap()]["extra"] = self.extra_checklist.clone().unwrap();
    }

    // Must be boolean only. 
    let checklist: Result<Vec<bool>, serde_json::Error> = 
      serde_json::from_value(self.checklist.clone().unwrap());
    if checklist.is_err() { error!("checklist_dto checklist not bool only."); 
      return Err(VEC_BOOL_ONLY.to_owned()); }
    new_serde["pipelines"][self.stage_index][self.pipeline_index]
      [self.cycle_index.unwrap()]["data"] = json!(checklist.unwrap());
    Ok(new_serde)
  }

  fn delete_response(&self, old_serde: Value) -> Result<Value, String> {
    if self.cycle_index.is_none() { error!("checklist_dto edit cycle index null"); 
      return Err(CYCLE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    if new_serde["pipelines"][self.stage_index].is_null() { 
      error!("checklist_dto delete stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index].clone();
    if pipeline.is_null() {
      error!("checklist_dto delete pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    let cycles = pipeline[self.cycle_index.unwrap()].clone();
    if cycles.is_null() {
      error!("checklist_dto delete cycle"); return Err(OOB_CYCLE_IDX.to_owned()); }
    
    let mut c = cycles["data"].as_array().unwrap().clone();
    c = c.iter_mut()
      .map(|_| json!(false))  // map to false instead of "".
      .collect();
    new_serde["pipelines"][self.stage_index][self.pipeline_index]
      [self.cycle_index.unwrap()]["data"] = json!(c);
    
    // This for extra
    new_serde["pipelines"][self.stage_index][self.pipeline_index]
      [self.cycle_index.unwrap()]["extra"] = json!(null);

    Ok(new_serde)
  }
}