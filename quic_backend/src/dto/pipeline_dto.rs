use crate::*;

use self::messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitPipeline {
  pub(crate) filename: String,
  pub(crate) stage_index: usize,
  pub(crate) pipeline_index: usize,
}

pub(crate) trait PipelineTrait {
  fn get_pipeline(&self, old_serde: Value) -> Result<Value, String>;
}

impl PipelineTrait for SubmitPipeline {
  fn get_pipeline(&self, old_serde: Value) -> Result<Value, String> {
    let stages = old_serde["stages"][self.stage_index].clone();
    if stages.is_null() { error!("pipeline_dto get_pipeline stages"); return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index].clone();
    if pipeline.is_null() { error!("pipeline_dto get_pipeline pipeline"); return Err(OOB_PIPELINE_IDX.to_owned()); }
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
      l1_script.push(json!(vec![""; pipelines[j]["others"].as_array().unwrap().len()]));
    }
    script.push(json!(l1_script));
  }

  json!(script)
}