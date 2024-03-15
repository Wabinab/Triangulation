// use clap::builder::Str;

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitStage {
  pub(crate) filename: String,
  pub(crate) stages: Option<Value>,
  pub(crate) stage_index: Option<usize>  // delete only. 
}

pub(crate) trait StageTrait {
  /// Update ALL existing stages (when click save button)
  /// Same for add new, also use this function. 
  fn edit_stage(&self, old_serde: Value) -> Value;

  /// Delete existing stage, given stage_index. 
  /// Return None if error. 
  fn delete_stage(&self, old_serde: Value) -> Result<Value, String>;
}

impl StageTrait for SubmitStage {
  fn edit_stage(&self, old_serde: Value) -> Value {
    let mut new_serde = old_serde.clone();

    // Loop through submitted stages. Any extra, add to the back. 
    let bind_stages = self.stages.clone().unwrap();
    let stages = bind_stages.as_array().unwrap();
    let mut new_stages: Vec<Value> = Vec::new();

    for (i, stage) in stages.iter().enumerate() {
      let mut item = stage.clone();
      item["pipeline"] = old_serde["stages"][i]["pipeline"].clone();
      if item["pipeline"].is_null() { item["pipeline"] = json!([]); }
      new_stages.push(item);
    }

    new_serde["stages"] = json!(new_stages);
    new_serde
  }

  fn delete_stage(&self, old_serde: Value) -> Result<Value, String> {
    let mut new_serde = old_serde.clone();

    let bind_stages = old_serde["stages"].clone();
    let mut stages = bind_stages.as_array().unwrap().clone();
    if self.stage_index.unwrap() >= stages.len() { error!("stage_dto delete"); return Err("Invalid Stage Index.".to_owned()); }

    stages.remove(self.stage_index.unwrap());

    new_serde["stages"] = json!(stages);
    Ok(new_serde)
  }
}