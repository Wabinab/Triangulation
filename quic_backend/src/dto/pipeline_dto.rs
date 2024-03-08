use crate::*;

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
    if stages.is_null() { return Err("Out of Bound stage index.".to_owned()); }
    let pipeline = stages["pipeline"][self.pipeline_index].clone();
    if pipeline.is_null() { return Err("Out of Bound pipeline index.".to_owned()); }
    Ok(pipeline)
  }
}