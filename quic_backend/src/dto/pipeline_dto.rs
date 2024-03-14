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
    if stages.is_null() { return Err(OOB_STAGE_IDX.to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index].clone();
    if pipeline.is_null() { return Err(OOB_PIPELINE_IDX.to_owned()); }
    Ok(pipeline)
  }
}


// =======================================================
/// Used to generate empty pipeline, to allow easier "insert" later. 
/// Actually, "pipeline" refers to "stage" and "pipeline" here. 
pub(crate) fn gen_empty_pipeline(data: Value) -> Result<Value, String> {
  // let mut filepath = get_savepath(data_path);
  // filepath.push(gen_filename(TEMPLATE_NAME.to_owned(), t_uuid, Some(t_ver)));
  // let data = retrieve_decompress_fullpath(filepath.clone());
  // if data.is_err() { return Err(data.unwrap_err()); }
  // let data = data.unwrap();

  let mut script: Vec<Value> = Vec::new();
  let stages = data["stages"].as_array().unwrap();
  for i in 0..stages.len() {
    let mut l1_script: Vec<Value> = Vec::new();
    let pipelines = stages[i]["pipeline"].as_array().unwrap();
    for j in 0..pipelines.len() {
      info!("{:#?}", pipelines[j]);
      l1_script.push(json!(vec![""; pipelines[j]["others"].as_array().unwrap().len()]));
    }
    script.push(json!(l1_script));
  }

  Ok(json!(script))
}