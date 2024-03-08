use crate::*;

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
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { return Err("Out of Bound stage index.".to_owned()); }

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
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { return Err("Out of Bound stage index.".to_owned()); }
    if self.reminder_index.is_none() { return Err("Reminder Index cannot be null.".to_owned()); }
    let pipeline = stages["pipeline"][self.reminder_index.unwrap()].clone();
    if pipeline.is_null() { return Err("Out of Bound reminder index.".to_owned()); }

    let data = json!({
      "ty": REMINDER_TYPE,
      "title": self.title.clone().unwrap(),
      "others": self.question.clone().unwrap()
    });

    new_serde["stages"][self.stage_index]["pipeline"][self.reminder_index.unwrap()] = data;
    Ok(new_serde)
  }

  fn delete_reminder(&self, old_serde: Value) -> Result<Value, String> {
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { return Err("Out of Bound stage index.".to_owned()); }
    if self.reminder_index.is_none() { return Err("Reminder Index cannot be null.".to_owned()); }
    let pipeline = stages["pipeline"][self.reminder_index.unwrap()].clone();
    if pipeline.is_null() { return Err("Out of Bound reminder index.".to_owned()); }

    let bind_pipeline = stages["pipeline"].clone();
    let mut pipelines = bind_pipeline.as_array().unwrap().clone();
    pipelines.remove(self.reminder_index.unwrap());

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }
}