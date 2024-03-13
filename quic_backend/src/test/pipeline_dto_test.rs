use serde_json::Value;

use crate::{messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX}, pipeline_dto::{PipelineTrait, SubmitPipeline}};

fn get_old_serde() -> Value {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {"ty": 0, "title": "Title 1", "others": "array/whatever as it is"},
          {"ty": 0, "title": "Title 2", "others": "array/whatever as it is"}
        ]},
        {"name": "Stage 2", "pipeline": [

        ]}
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// =================================================
#[test]
fn test_pipeline_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_pipeline(old_serde).unwrap();
  assert_eq!(edited_serde["title"], "Title 2");
}

#[test]
fn test_pipeline_invalid_stage_index() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 500, 
    "pipeline_index": 0
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_pipeline(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}


#[test]
fn test_pipeline_invalid_pipeline_index() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0, 
    "pipeline_index": 500
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_pipeline(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}