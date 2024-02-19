use crate::*;
use self::json::helper::{get_by_locale, find_by_id};

// Submission Templates
// Note: Stage DOES NOT include the pipelines. 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitEditStage {
  pub(crate) filename: String,
  pub(crate) stages: Value
}

pub(crate) trait SubmitStageTrait {
  fn to_serde(&self, old_serde: Value) -> Value;
}

impl SubmitStageTrait for SubmitEditStage {
  /// old_serde should be fetched from .json.zl file and pass in here. 
  fn to_serde(&self, old_serde: Value) -> Value {
    let mut new_serde = old_serde.clone();

    // To account for possibility of deletion, we need to assign the whole stuff. 
    // Then, we'll move the pipeline from old_serde to new stages. 
    let arr_stages = self.stages.as_array().unwrap();
    let mut new_stages: Vec<Value> = Vec::new();

    for item in arr_stages {
      let mut item = item.clone();
      item["pipeline"] = match find_by_id(old_serde["stages"].clone(), "step", item["step"].as_u64().unwrap()) {
        Some(val) => val["pipeline"].clone(),
        None => json!([])
      };
      new_stages.push(item);
    }

    new_serde["stages"] = json!(new_stages);
    return new_serde;
  }
}

// ======================================================
// Using JSON
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Stage {
  pub(crate) step: u64,  // because Value can only support u64
  pub(crate) name: String,
  pub(crate) pipeline: Value
}

// pub(crate) trait StageTrait {
//   fn to_serde(&self, )
// }