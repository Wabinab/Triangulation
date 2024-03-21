use crate::*;

use self::{file::gen_filename, versioning::{get_ver, get_verpath}};

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
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitGetTemplate {
  pub(crate) filename: String
}


// ========================================
pub(crate) fn to_nlist_temp(old_serde: Value) -> TemplateNList {
  // info!("{:#?}", old_serde);
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

// ==============================================
pub(crate) fn to_nameonly(old_serde: Value) -> TemplateNameonly {
  serde_json::from_value(old_serde).unwrap()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TemplateNameonly {
  pub(crate) name: String,
  pub(crate) uuid: String
}

// ================================================
pub(crate) fn to_basic_template(old_serde: Value) -> TemplateBasic {
  serde_json::from_value(old_serde).unwrap()
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TemplateBasic {
  pub(crate) name: String,
  pub(crate) uuid: String,
  pub(crate) description: String
}

// ================================================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitTemplateVer {
  pub(crate) t_uuid: String
}

pub(crate) trait TemplateVerTrait {
  fn get_version(&self, data_path: PathBuf) -> Result<Version, String>;
}

impl TemplateVerTrait for SubmitTemplateVer {
  fn get_version(&self, data_path: PathBuf) -> Result<Version, String> {
    let ver_path = get_verpath(data_path);
    let filename = gen_filename(
      TEMPLATE_NAME.to_owned(), self.t_uuid.to_owned(), None);
    get_ver(ver_path, filename)
  }
}