use crate::*;

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
  fn new_project(&self, uuid: String, version: usize) -> Result<Value, String>;

  /// Use to edit existing file: name, description, and change template version. 
  /// We don't check upgrade or downgrade; but humans should keep track manually.
  /// We also don't allow upgrade only; because they can downgrade if needed be.
  fn edit_template(&self, old_serde: Value) -> Result<Value, String>;
}

impl ProjectTrait for SubmitProject {
  fn new_project(&self, uuid: String, version: Version) -> Result<Value, String> {
      // Actually it's template uuid, but end user can't understand. 
      if self.template_uuid.is_none() { return Err("Template must not be null.".to_string()); }
      Ok(json!({
        "name": self.name.clone(),
        "uuid": uuid,
        "description": self.description.clone(),
        "t_uuid": self.template_uuid.clone(),
        "t_ver": version
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