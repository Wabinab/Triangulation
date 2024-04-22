use crate::{messages::OOB_CYCLE_IDX, *};

use self::{file::gen_filename, messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX}};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitPipeline {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: usize,
  pub(crate) cycle_index: Option<usize>
}

pub(crate) trait PipelineTrait {
  // This is from template pipeline. 
  fn get_pipeline(&self, old_serde: Value) -> Result<Value, String>;

  // This is from project pipeline (a.k.a. response)
  fn get_response(&self, old_serde: Value) -> Result<Value, String>;
}

impl PipelineTrait for SubmitPipeline {
  fn get_pipeline(&self, old_serde: Value) -> Result<Value, String> {
    let stages = old_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("pipeline_dto get_pipeline stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index].clone();
    if pipeline.is_null() { error!("pipeline_dto get_pipeline pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    Ok(pipeline)
  }

  fn get_response(&self, old_serde: Value) -> Result<Value, String> {
    let stages = old_serde["pipelines"][self.stage_index].clone();
    if stages.is_null() { error!("pipeline_dto get_response stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let response = stages[self.pipeline_index].clone();
    if response.is_null() { error!("pipeline_dto get_response response"); return Err(OOB_PIPELINE_IDX.to_owned()); } 

    if self.cycle_index.is_some() {
      let resp2 = response[self.cycle_index.unwrap()]["data"].clone();
      if resp2.is_null() { error!("pipeline_dto get_response cycle not null but resp2 null."); return Err(OOB_CYCLE_IDX.to_owned()); }
      return Ok(resp2);
    }

    Ok(response)
  }
}


// Submit pipeline from project side, so use t_uuid and t_ver instead. 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitPipelineViaProj {
  pub(crate) t_uuid: String,
  pub(crate) t_ver: usize,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: usize
}

pub(crate) trait PipelineViaProjTrait {
  fn get_filename(&self) -> String;  // get filename, not the actual file content. 
  fn get_pipeline(&self, old_serde: Value) -> Result<Value, String>;
}

impl PipelineViaProjTrait for SubmitPipelineViaProj {
  fn get_filename(&self) -> String {
    return gen_filename(TEMPLATE_NAME.to_owned(), 
      self.t_uuid.clone(), Some(self.t_ver.clone()));
  }

  fn get_pipeline(&self, old_serde: Value) -> Result<Value, String> {
    let stages = old_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("pipeline_dto_proj get_pipeline stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index].clone();
    if pipeline.is_null() { error!("pipeline_dto_proj get_pipeline pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
    Ok(pipeline)
  }
}


// =======================================================
/// Used to generate empty pipeline, to allow easier "insert" later. 
/// Actually, "pipeline" refers to "stage" and "pipeline" here. 
/// 
/// IF THIS PANIC, it must be others who have problem than this. 
/// One MUST have "stages"; and each stages MUST have "pipeline";
/// and each pipeline MUST have "others".
pub(crate) fn gen_empty_pipeline(data: Value) -> Value {
  let mut script: Vec<Value> = Vec::new();
  let stages = data["stages"].as_array().unwrap();
  for i in 0..stages.len() {
    let mut l1_script: Vec<Value> = Vec::new();
    let pipelines = stages[i]["pipeline"].as_array().unwrap();
    for j in 0..pipelines.len() {
      let mut l2_script: Vec<Value> = Vec::new();
      l2_script.push(json!({
        "name": "0",
        "data": vec![""; pipelines[j]["others"].as_array().unwrap().len()]
      }));
      l1_script.push(json!(l2_script));
    }
    script.push(json!(l1_script));
  }

  json!(script)
}