// We only keep latest 100 transactions for kelly criterion. 
// Anything new will "shift" out position 0's and throw it away. 
use crate::{messages::{PIPELINE_IDX_CANNOT_NULL, OOB_PIPELINE_IDX, OOB_STAGE_IDX, TITLE_NONE, TRANSACTION_NONE}, *};

use self::{reminder_dto::ReminderTrait, response_dto::ResponseTrait};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitKelly {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: Option<usize>,
  pub(crate) title: Option<String>,
  pub(crate) transactions: Option<Vec<Transaction>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Transaction {
  pub(crate) coin: String,
  pub(crate) buy: bool,  // true = buy, false = sell. 
  pub(crate) price: Decimal,
  pub(crate) amt: Decimal,
  pub(crate) price_1: Option<Decimal>,
  pub(crate) amt_1: Option<Decimal>,
  pub(crate) pred_prob: Decimal
}

impl ReminderTrait for SubmitKelly {
  // New kelly
  fn new_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("kelly_dto new title is none."); return Err(TITLE_NONE.to_owned()); }
    // if self.transactions.is_none() { error!("kelly_dto new transactions is none."); return Err(TRANSACTION_NONE.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("kelly_dto new stages"); return Err(OOB_STAGE_IDX.to_owned()); }

    let mut pipelines = stages["pipeline"].as_array().unwrap().clone();
    let data = json!({
      "ty": KELLY_TYPE,
      "title": self.title.clone().unwrap(),
      "others": []
    });
    pipelines.push(data);

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }

  fn edit_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.title.is_none() { error!("kelly_dto edit title is none."); return Err(TITLE_NONE.to_owned()); }
    // if self.transactions.is_none() { error!("kelly_dto edit transactions is none."); return Err(TRANSACTION_NONE.to_owned()); }
    if self.pipeline_index.is_none() { error!("kelly_dto edit pipeline_index is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("kelly_dto edit stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() { error!("kelly_dto edit pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let data = json!({
      "ty": KELLY_TYPE,
      "title": self.title.clone().unwrap(),
      "others": pipeline["others"].clone()
    });

    new_serde["stages"][self.stage_index]["pipeline"][self.pipeline_index.unwrap()] = data;
    Ok(new_serde)
  }

  fn delete_reminder(&self, old_serde: Value) -> Result<Value, String> {
    if self.pipeline_index.is_none() { error!("kelly_dto edit kelly_idx is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();

    let stages = new_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("kelly_dto delete stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() { error!("kelly_dto delete pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }

    let bind_pipeline = stages["pipeline"].clone();
    let mut pipelines = bind_pipeline.as_array().unwrap().clone();
    pipelines.remove(self.pipeline_index.unwrap());

    new_serde["stages"][self.stage_index]["pipeline"] = json!(pipelines);
    Ok(new_serde)
  }
}

impl ResponseTrait for SubmitKelly {
  fn edit_response(&self, old_serde: Value) -> Result<Value, String> {
    if self.pipeline_index.is_none() { error!("kelly_dto edit-response kelly_idx is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    if self.transactions.is_none() { error!("kelly_dto edit-response transactions is none."); return Err(TRANSACTION_NONE.to_owned()); }
    let mut new_serde = old_serde.clone();
    if new_serde["pipelines"][self.stage_index].is_null() {
      error!("kelly_dto edit-response stage idx oob"); return Err(OOB_STAGE_IDX.to_owned());
    }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() {
      error!("kelly_dto edit-response pipeline idx oob"); return Err(OOB_PIPELINE_IDX.to_owned());
    }
    // No check length unlike normal response. 
    new_serde["pipelines"][self.stage_index][self.pipeline_index.unwrap()] = json!(self.transactions.as_ref().unwrap().clone());
    Ok(new_serde)
  }

  fn delete_response(&self, old_serde: Value) -> Result<Value, String> {
    if self.pipeline_index.is_none() { error!("kelly_dto edit-response kelly_idx is null."); return Err(PIPELINE_IDX_CANNOT_NULL.to_owned()); }
    let mut new_serde = old_serde.clone();
    if new_serde["pipelines"][self.stage_index].is_null() {
      error!("kelly_dto delete-response stage idx oob"); return Err(OOB_STAGE_IDX.to_owned());
    }
    let pipeline = new_serde["pipelines"][self.stage_index][self.pipeline_index.unwrap()].clone();
    if pipeline.is_null() {
      error!("kelly_dto delete-response pipeline idx oob"); return Err(OOB_PIPELINE_IDX.to_owned());
    }
    new_serde["pipelines"][self.stage_index][self.pipeline_index.unwrap()] = json!([]);
    Ok(new_serde)
  }
}