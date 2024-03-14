use crate::{pipeline_dto::gen_empty_pipeline, *};

use self::messages::TEMPLATE_CANNOT_NULL;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitProject {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) template_uuid: Option<String>,
  pub(crate) version: Option<Version>,  // needed but not necessarily submitted.
  pub(crate) filename: Option<String>
}

pub(crate) trait ProjectTrait {
  /// Use when not yet create file.
  fn new_project(&self, uuid: String, version: Version, template_serde: Value) -> Result<Value, String>;

  /// Use to edit existing file: name, description, and change template version. 
  /// We don't check upgrade or downgrade; but humans should keep track manually.
  /// We also don't allow upgrade only; because they can downgrade if needed be.
  fn edit_template(&self, old_serde: Value) -> Result<Value, String>;
}

impl ProjectTrait for SubmitProject {
  fn new_project(&self, uuid: String, version: Version, template_serde: Value) -> Result<Value, String> {
      // Actually it's template uuid, but end user can't understand. 
      if self.template_uuid.is_none() { return Err(TEMPLATE_CANNOT_NULL.to_string()); }
      Ok(json!({
        "name": self.name.clone(),
        "uuid": uuid,
        "description": self.description.clone(),
        "t_uuid": self.template_uuid.clone(),
        "t_ver": version,
        "pipelines": gen_empty_pipeline(template_serde)
        // "pipelines": []
      }))
  }

  fn edit_template(&self, old_serde: Value) -> Result<Value, String> {
      let mut new_serde = old_serde.clone();
      new_serde["name"] = json!(self.name.clone());
      new_serde["description"] = json!(self.description.clone());
      if self.version.is_some() { new_serde["t_ver"] = json!(self.version.clone().unwrap()); }
      return Ok(new_serde);
  }
}

// =========================================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitGetProject {
  pub(crate) filename: String
}

// =========================================
pub(crate) fn to_nlist_proj(old_serde: Value) -> ProjectNList {
  serde_json::from_value(old_serde).unwrap()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProjectNList {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) uuid: String,
  pub(crate) t_uuid: String,
  pub(crate) t_ver: Version
}