use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitTemplate {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) filename: Option<String>
}

pub(crate) trait TemplateTrait {
  /// Use when not yet create file. 
  fn new_template(&self, uuid: String) -> Value;

  /// Use to edit existing file: name and description. 
  fn edit_template(&self, old_serde: Value) -> Value;

  // Delete template no need, since it's just deleting the whole file. 
  // Though, need to check for project dependencies before delete. 
}

impl TemplateTrait for SubmitTemplate {
  fn new_template(&self, uuid: String) -> Value {
      json!({
        "name": self.name.clone(),
        "uuid": uuid,
        "description": self.description.clone(),
        "stages": json!([])
      })
  }

  fn edit_template(&self, old_serde: Value) -> Value {
    let mut new_serde = old_serde.clone();
    new_serde["name"] = json!(self.name.clone());
    new_serde["description"] = json!(self.description.clone());
    return new_serde;
  }
}


// ========================================
pub(crate) fn to_nlist(old_serde: Value) -> TemplateNList {
  serde_json::from_value(old_serde).unwrap()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TemplateNList {
  pub(crate) name: String,
  pub(crate) uuid: String,
  pub(crate) description: String,
  pub(crate) stages: Vec<StageNList>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StageNList {
  pub(crate) name: String,
  pub(crate) pipeline: Vec<PipelineNList>
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PipelineNList {
  pub(crate) ty: u64,
  pub(crate) title: String
}