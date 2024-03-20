use serde_json::Value;

use crate::{response_dto::{ResponseTrait, SubmitResponse}, messages::{ANSWER_NULL, LEN_PIPELINE_NOT_MATCH}};

fn get_old_serde() -> Value {
  let c = r#" {
    "name": "With Special Question",
    "description": "This is version 3 of template, with special questions. ",
    "t_uuid": "018e1be4-582c-70bf-8972-d0e5c4786f2a",
    "uuid": "018e5065-393a-7cc9-beca-9945f961068f",
    "t_ver": 2,
    "pipelines": [
      [
        [
          "",
          "",
          ""
        ],
        [
          ""
        ],
        [
          "short response",
          "long response",
          1,
          [0, 1],
          3,
          [3, 4, 0],
          [[1, 2], [2]],
          "2024-03-20T11:23"
        ]
      ]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// =============================================
#[test]
fn test_edit_correct_1() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "answer": [
      1,
      [[1, 2], [2], [0, 1, 2]],
      [1, 0]
    ]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][0].clone();
  let old_ooi = old_serde["pipelines"][0][0].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len());
  assert_ne!(ooi[0], old_ooi[0]);
  assert_eq!(ooi[0], 1);
  assert_eq!(ooi[1].as_array().unwrap().len(), 3);
  // Real difficult to test without permutations. 
  assert_eq!(ooi[2].as_array().unwrap().len(), 2);
  assert_eq!(ooi[2][0], 1);
  assert_eq!(ooi[2][1], 0);
}

#[test]
fn test_edit_len_diff_err() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "answer": [
      1,
      [[1, 2], [2], [0, 1, 2]],
      [1, 0],
      "this shouldn't be here"
    ]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == LEN_PIPELINE_NOT_MATCH.to_owned()));
}

#[test]
fn test_edit_changed_existing() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 2,
    "answer": [
      "shorter response",
      "longer response",
      1,
      [0, 1],
      5,
      [3, 4, 0],
      [[1, 2], [2]],
      "2024-03-20T11:23"
    ]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][2].clone();
  let old_ooi = old_serde["pipelines"][0][2].clone();
  assert_ne!(ooi[0], old_ooi[0]);
  assert_ne!(ooi[1], old_ooi[1]);
  assert_eq!(ooi[2], old_ooi[2]);
  assert_eq!(ooi[0], "shorter response");
  assert_eq!(ooi[4], 5);
}

#[test]
fn test_edit_same() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 2,
    "answer": [
      "short response",
      "long response",
      1,
      [0, 1],
      3,
      [3, 4, 0],
      [[1, 2], [2]],
      "2024-03-20T11:23"
    ]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  assert_eq!(edited_serde["pipelines"][0][2], old_serde["pipelines"][0][2]);
}

#[test]
fn test_delete_existing() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 2
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][2].clone();
  let old_ooi = old_serde["pipelines"][0][2].clone();
  assert_ne!(ooi, old_ooi);
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len());
  for i in 0..=7 { assert_eq!(ooi[i], ""); }
}

#[test]
fn test_delete_nonexistent() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  assert_eq!(edited_serde["pipelines"][0][0], old_serde["pipelines"][0][0]);
}

#[test]
fn test_answer_null_err() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 2
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == ANSWER_NULL.to_owned()));
}