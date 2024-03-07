use serde_json::{json, Value};

use crate::stage_dto::{StageTrait, SubmitStage};

use super::*;

#[test]
fn test_edit_stage() {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {"ty": 0, "title": "...", "others": "array/whatever as it is"},
          {"ty": 0, "title": "...", "others": "array/whatever as it is"}
        ]},
        {"name": "Stage 2", "pipeline": [

        ]}
    ]
  }"#;
  let old_serde: Value = serde_json::from_str(&c).unwrap();

  let d = r#"{
    "filename": "...",
    "stages": [
			{"name": "New Stage 1"},
			{"name": "New Stage 2"},
			{"name": "New Stage 3"}
    ]
  }"#;
  let submit: SubmitStage = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_stage(old_serde.clone());

  assert_eq!(old_serde["stages"].as_array().unwrap().len(), 2);
  assert_eq!(edited_serde["stages"].as_array().unwrap().len(), 3);
  assert_eq!(edited_serde["stages"][0]["pipeline"].as_array().unwrap().len(), 2);
  assert_eq!(edited_serde["stages"][2]["pipeline"], json!([]));
  assert_ne!(old_serde["stages"][0]["name"], edited_serde["stages"][0]["name"]);
}


#[test]
fn test_delete_stage_valid_id() {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {"ty": 0, "title": "...", "others": "array/whatever as it is"},
          {"ty": 0, "title": "...", "others": "array/whatever as it is"}
        ]},
        {"name": "Stage 2", "pipeline": []},
        {"name": "Stage 3", "pipeline": []}
    ]
  }"#;
  let old_serde: Value = serde_json::from_str(&c).unwrap();

  let d = r#"{
    "filename": "...",
    "stage_index": 1
  }"#;
  let submit: SubmitStage = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_stage(old_serde.clone()).unwrap();
  assert_eq!(edited_serde["stages"].as_array().unwrap().len(), old_serde["stages"].as_array().unwrap().len() - 1);
  assert_eq!(edited_serde["stages"][0]["name"].as_str().unwrap(), "Stage 1");
  assert_eq!(edited_serde["stages"][1]["name"].as_str().unwrap(), "Stage 3");
}


#[test]
fn test_delete_stage_invalid_id() {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {"ty": 0, "title": "...", "others": "array/whatever as it is"},
          {"ty": 0, "title": "...", "others": "array/whatever as it is"}
        ]},
        {"name": "Stage 2", "pipeline": []},
        {"name": "Stage 3", "pipeline": []}
    ]
  }"#;
  let old_serde: Value = serde_json::from_str(&c).unwrap();

  let d = r#"{
    "filename": "...",
    "stage_index": 500
  }"#;
  let submit: SubmitStage = serde_json::from_str(&d).unwrap();

  let edited_stage = submit.delete_stage(old_serde);
  assert!(edited_stage.is_err());
}