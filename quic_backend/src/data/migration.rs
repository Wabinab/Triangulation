use log::error;
/// This is use to migrate data when versioning changes. 
/// To keep things simple, we'll move data block by block, if it isn't empty string. 

use serde_json::{json, Value};

use crate::{messages::OOB_MIGRATE, pipeline_dto::gen_empty_pipeline};

pub(crate) fn migrate_data(new_templ_serde: Value, pipelines: Value) -> Result<Value, String> {
  let mut templ = gen_empty_pipeline(new_templ_serde);
  // let pipelines = proj_serde["pipelines"].clone();
  let empty = json!("");
  
  for (i, el_i) in pipelines.as_array().unwrap().iter().enumerate() {
    for (j, el_j) in el_i.as_array().unwrap().iter().enumerate() {
      for (k, el_k) in el_j.as_array().unwrap().iter().enumerate() {
        let ooi = templ[i][j][k].clone();
        if !ooi.is_null() { templ[i][j][k] = el_k.clone(); }
        else if ooi.is_null() && el_k == &empty { continue; }
        else { error!("migrate_data"); return Err(OOB_MIGRATE.to_owned()); }
      }
    }
  }

  Ok(templ)
}

/// Unsafe migration. 
/// Any data outside bound will be lost forever! 
pub(crate) fn unsafe_migrate_data(new_templ_serde: Value, pipelines: Value) -> Result<Value, String> {
  let mut templ = gen_empty_pipeline(new_templ_serde);
  // let pipelines = proj_serde["pipelines"].clone();
  
  for (i, el_i) in pipelines.as_array().unwrap().iter().enumerate() {
    for (j, el_j) in el_i.as_array().unwrap().iter().enumerate() {
      for (k, el_k) in el_j.as_array().unwrap().iter().enumerate() {
        if !templ[i][j][k].is_null() { templ[i][j][k] = el_k.clone(); }
      }
    }
  }

  Ok(templ)
}