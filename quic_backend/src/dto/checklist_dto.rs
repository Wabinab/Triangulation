use crate::{messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX, PIPELINE_IDX_CANNOT_NULL, TITLE_NONE}, *};

use self::reminder_dto::ReminderTrait;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitChecklist {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: Option<usize>,
  pub(crate) title: Option<String>,
  pub(crate) checklist: Option<Vec<String>>  // share between compulsory and extra.
}

impl ReminderTrait for SubmitChecklist {
  fn new_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("checklist_dto title is none."); return Err(TITLE_NONE.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("checklist_dto new stages"); return Err(OOB_STAGE_IDX.to_owned()); }

    let mut pipelines = stages["pipeline"].as_array().unwrap().clone();
    let data = json!({
      "ty": CHECKLIST_TYPE,
      "title": self.title.clone().unwrap(),
      "others": []
    });
    pipelines.push(data);

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }

  fn edit_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("checklist_dto title is none."); return Err(TITLE_NONE.to_owned()); }
    if self.pipeline_index.is_none() { error!("checklist_dto edit pipeline_index is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("checklist_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() { error!("checklist_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let data = json!({
      "ty": CHECKLIST_TYPE,
      "title": self.title.clone().unwrap(),
      "others": pipeline["others"].clone()
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