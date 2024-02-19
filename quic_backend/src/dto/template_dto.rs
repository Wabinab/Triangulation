use crate::*;

use self::json::helper::get_by_locale;

// Submission templates
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitEditTemplate {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) locale: String,
  pub(crate) filename: String,  // for edit only. 
}

pub(crate) trait SubmitTemplateTrait {
  fn to_new_template(&self, uuid: String, stages: Value) -> Template;
  fn to_new_serde(&self, uuid: String, stages: Value) -> Value;
  fn to_serde(&self, old_serde: Value) -> Value;
}

impl SubmitTemplateTrait for SubmitEditTemplate {
  fn to_new_template(&self, uuid: String, stages: Value) -> Template {
    Template {
      name: self.name.clone(),
      uuid: uuid,
      description: self.description.clone(),
      stages: stages
    }
  }

  fn to_new_serde(&self, uuid: String, stages: Value) -> Value {
      let data = json!({
        "name": { self.locale.clone(): self.name.clone() },
        "uuid": uuid,
        "description": { self.locale.clone(): self.description.clone() },
        "stages": stages
      });

      data
  }

  fn to_serde(&self, old_serde: Value) -> Value {
    let mut new_serde = old_serde.clone();
    new_serde["name"][self.locale.clone()] = json!(self.name.clone());
      // uuid never change since created new.
    new_serde["description"][self.locale.clone()] = json!(self.description.clone());
    // Stages shouldn't change, so pass it on. 
    return new_serde;
  }
}

// =======================
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitGetTemplate {
  pub(crate) filename: String
}

// ===========================================================
// Using JSON
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Template {
  pub(crate) name: String,
  pub(crate) uuid: String,
  pub(crate) description: String,
  pub(crate) stages: Value
}

pub(crate) trait TemplateTrait {
  fn to_serde_new(&self, locale: String) -> Value;
  fn to_serde_existing(&self, old_serde: Value, locale: String) -> Value;
  fn from_serde(&self, serde: Value, locale: String) -> Template;
}

impl TemplateTrait for Template {
  fn to_serde_new(&self, locale: String) -> Value {
      // serde_json::to_value(&self)  // wrong, forget about locale. 
      let data = json!({
        "name": {
          locale.clone(): self.name
        },
        "uuid": self.uuid,
        "description": {
          locale: self.description
        },
        "stages": self.stages
      });

      data
  }

  fn to_serde_existing(&self, old_serde: Value, locale: String) -> Value {
      let mut new_serde = old_serde.clone();
      new_serde["name"][locale.clone()] = json!(self.name.clone());
      // uuid never change since created new.
      new_serde["description"][locale] = json!(self.description.clone());
      // Stages shouldn't change, so pass it on. 
      return new_serde;
  }

  fn from_serde(&self, serde: Value, locale: String) -> Template {
      Template {
        name: get_by_locale(serde["name"].clone(), locale.clone()),
        uuid: serde["uuid"].to_string(),
        description: get_by_locale(serde["description"].clone(), locale),
        stages: serde["stages"].clone()
      }
  }
}


// ============================================================
// Using Diesel
// #[derive(Queryable, Selectable, Deserialize, Insertable)]
// #[diesel(table_name = crate::schema::Template)]
// pub struct DieselTemplate<'a> {
//   pub(crate) 
// }