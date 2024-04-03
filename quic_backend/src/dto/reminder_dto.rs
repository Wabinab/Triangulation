use crate::{messages::{OOB_PIPELINE_IDX, QUESTION_NONE, TITLE_NONE}, *};

use self::messages::{OOB_STAGE_IDX, REMINDER_IDX_CANNOT_NULL};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitReminder {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) reminder_index: Option<usize>,
  pub(crate) title: Option<String>,
  pub(crate) question: Option<Value>
}

pub(crate) trait ReminderTrait {
  /// Create new reminder.
  fn new_reminder(&self, old_serde: Value) -> Result<Value, String>;

  /// Edit existing reminder. 
  fn edit_reminder(&self, old_serde: Value) -> Result<Value, String>;

  /// Delete a reminder. 
  fn delete_reminder(&self, old_serde: Value) -> Result<Value, String>;
}

impl ReminderTrait for SubmitReminder {
  fn new_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("reminder_dto new title is none."); return Err(TITLE_NONE.to_owned()); }
    if self.question.is_none() { error!("reminder_dto new question is none."); return Err(QUESTION_NONE.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("reminder_dto new stages"); return Err(OOB_STAGE_IDX.to_owned()); }

    let mut pipelines = stages["pipeline"].as_array().unwrap().clone();
    let data = json!({
      "ty": REMINDER_TYPE,
      "title": self.title.clone().unwrap(),
      "others": self.question.clone().unwrap()
    });
    pipelines.push(data);

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }

  fn edit_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("reminder_dto edit title is none."); return Err(TITLE_NONE.to_owned()); }
    if self.question.is_none() { error!("reminder_dto edit question is none."); return Err(QUESTION_NONE.to_owned()); }
    if self.reminder_index.is_none() { error!("reminder_dto edit reminder_idx is null."); return Err(REMINDER_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("reminder_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.reminder_index.unwrap()].clone();
    if pipeline.is_null() { error!("reminder_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let data = json!({
      "ty": REMINDER_TYPE,
      "title": self.title.clone().unwrap(),
      "others": self.question.clone().unwrap()
    });

    new_serde["stages"][self.stage_index]["pipeline"][self.reminder_index.unwrap()] = data;
    Ok(new_serde)
  }

  fn delete_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.reminder_index.is_none() { error!("reminder_dto delete stages"); return Err(REMINDER_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("reminder_dto delete stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.reminder_index.unwrap()].clone();
    if pipeline.is_null() { error!("reminder_dto delete pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let bind_pipeline = stages["pipeline"].clone();
    let mut pipelines = bind_pipeline.as_array().unwrap().clone();
    pipelines.remove(self.reminder_index.unwrap());

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }
}