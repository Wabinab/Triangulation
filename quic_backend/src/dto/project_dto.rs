use crate::{messages::VER_TEMP_NONE, pipeline_dto::gen_empty_pipeline, *};

use self::{messages::TEMPLATE_CANNOT_NULL, migration::{migrate_data, unsafe_migrate_data}};

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
  fn edit_project(&self, old_serde: Value, new_templ_serde: Option<Value>) -> Result<Value, String>;
}

impl ProjectTrait for SubmitProject {
  fn new_project(&self, uuid: String, version: Version, template_serde: Value) -> Result<Value, String> {
      // Actually it's template uuid, but end user can't understand. 
      if self.template_uuid.is_none() { error!("new_project template uuid null."); return Err(TEMPLATE_CANNOT_NULL.to_string()); }
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

  fn edit_project(&self, old_serde: Value, new_templ_serde: Option<Value>) -> Result<Value, String> {
      let mut new_serde = old_serde.clone();
      new_serde["name"] = json!(self.name.clone());
      new_serde["description"] = json!(self.description.clone());
      if self.version.is_some() { 
        if new_templ_serde.is_none() { error!("{}", VER_TEMP_NONE); return Err(VER_TEMP_NONE.to_owned()); }
        let res = migrate_data(
          new_templ_serde.unwrap(), new_serde["pipelines"].clone());
        if res.is_err() { error!("project_dto edit_project migrate_data"); return Err(res.unwrap_err()); }
        new_serde["t_ver"] = json!(self.version.clone().unwrap()); 
        new_serde["pipelines"] = res.unwrap();
      }
      return Ok(new_serde);
  }
}

// =========================================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitProjVer {
  pub(crate) filename: String,
  pub(crate) version: Version
}

pub(crate) trait ProjVerTrait {
  fn edit_version(&self, old_serde: Value, new_templ_serde: Value) -> Result<Value, String>;
}

impl ProjVerTrait for SubmitProjVer {
  fn edit_version(&self, old_serde: Value, new_templ_serde: Value) -> Result<Value, String> {
    let mut new_serde = old_serde.clone();
    let res = unsafe_migrate_data(new_templ_serde, 
      new_serde["pipelines"].clone());
    if res.is_err() { error!("project_dto edit_version unsafe_migrate_data"); return Err(res.unwrap_err()); }
    new_serde["t_ver"] = json!(self.version.clone());
    new_serde["pipelines"] = res.unwrap();
    return Ok(new_serde);
  }
}

// =========================================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitGetProject {
  pub(crate) filename: String
}

// =========================================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitCloneProj {
  pub(crate) uuid: String
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

// ==============================================
pub(crate) fn to_basic_project(old_serde: Value) -> ProjectBasic {
  serde_json::from_value(old_serde).unwrap()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProjectBasic {
  pub(crate) name: String,
  pub(crate) uuid: String,
  pub(crate) description: String,
  pub(crate) t_name: Option<String>,
  pub(crate) t_ver: usize
}