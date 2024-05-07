use serde_json::{json, Value};

use crate::{messages::{ANS_NONE, CYCLE_AT_LEAST_ONE, CYCLE_IDX_CANNOT_NULL, CYCLE_NAME_NULL, LEN_PIPELINE_NOT_MATCH, OOB_CYCLE_IDX, OOB_PIPELINE_IDX, OOB_STAGE_IDX}, response_dto::{CycleTrait, ResponseTrait, SubmitResponse}};

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
          {
            "name": "Cycle 0",
            "data": ["", "", ""]
          }
        ],
        [
          {
            "name": "Cycle 0",
            "data": ["Nothing here???"]
          },
          {
            "name": "Cycle 1",
            "data": ["Something recorded here."]
          }
        ],
        [
          {
            "name": "Cycle 0",
            "data": [
              "short response",
              "long response",
              1,
              [0, 1],
              3,
              [3, 4, 0],
              [[1, 2], [2]],
              "2024-03-20T11:23"
            ]
          }
        ],
        [
          {
            "name": "0",
            "data": [true, false],
            "extra": [["Something extra"], [true]]
          }
        ]
      ]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// =============================================
#[test]
fn test_edit_correct() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "answer": [
      1,
      [[1, 2], [2], [0, 1, 2]],
      [1, 0]
    ]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][0][0]["data"].clone();
  let old_ooi = old_serde["pipelines"][0][0][0]["data"].clone();
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
    "cycle_index": 0,
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
    "cycle_index": 0,
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
  let ooi = edited_serde["pipelines"][0][2][0]["data"].clone();
  let old_ooi = old_serde["pipelines"][0][2][0]["data"].clone();
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
    "cycle_index": 0,
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
  assert_eq!(edited_serde["pipelines"][0][2][0]["data"], old_serde["pipelines"][0][2][0]["data"]);
}

#[test]
fn test_delete_existing() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 2,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][2][0]["data"].clone();
  let old_ooi = old_serde["pipelines"][0][2][0]["data"].clone();
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
    "pipeline_index": 0,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  assert_eq!(edited_serde["pipelines"][0][0][0]["data"], old_serde["pipelines"][0][0][0]["data"]);
}

// No longer true. 
// #[test]
// fn test_answer_null_err() {
//   let old_serde = get_old_serde();
//   let d = r#"{
//     "filename": "...",
//     "stage_index": 0,
//     "pipeline_index": 2,
//     "cycle_index": 0
//   }"#;
//   let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

//   let edited_serde = submit.edit_response(old_serde.clone());
//   assert!(edited_serde.is_err_and(|x| x == ANSWER_NULL.to_owned()));
// }

#[test]
fn test_edit_answer_and_cycle_name_both_none() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == ANS_NONE.to_owned()));
}

#[test]
fn test_edit_cycle_index_none() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "answer": ["this is the finisher"]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.clone().is_err_and(|x| x == CYCLE_IDX_CANNOT_NULL.to_owned()), "{:?}", edited_serde);
}

#[test]
fn test_edit_cycle_1_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 1,
    "cycle_name": "Whatever new name",
    "answer": ["this is the finisher"]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1][1].clone();
  let old_ooi = old_serde["pipelines"][0][1][1].clone();
  assert_ne!(ooi["data"], old_ooi["data"]);
  // assert_ne!(ooi["name"], old_ooi["name"]);
  // Name shouldn't change anymore. 
}

#[test]
fn test_edit_cycle_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 500,
    "cycle_name": "Whatever new name",
    "answer": ["this is the finisher"]
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_CYCLE_IDX.to_owned()));
}

#[test]
fn test_delete_cycle_index_none() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == CYCLE_IDX_CANNOT_NULL.to_owned()));
}

#[test]
fn test_delete_cycle_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 500
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_CYCLE_IDX.to_owned()));
}

#[test]
fn test_delete_name_not_reset() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 1
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1][1]["name"].clone();
  let old_ooi = old_serde["pipelines"][0][1][1]["name"].clone();
  assert_eq!(ooi, old_ooi);
  assert_ne!(ooi, "");
}

#[test]
fn test_add_new_cycle_ok() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_name": "New Cycle Added"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.add_new_cycle(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1].clone();
  let old_ooi = old_serde["pipelines"][0][1].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() + 1);
  let ooi2 = ooi[ooi.as_array().unwrap().len() - 1].clone();  // last added.
  let old_ooi2 = old_ooi[0].clone();
  assert_eq!(ooi2["name"], "New Cycle Added");
  assert_eq!(ooi2["data"], json!([""]));
  assert_ne!(old_ooi2["data"], json!([""]));
}

#[test]
fn test_add_cycle_cycle_name_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.add_new_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == CYCLE_NAME_NULL.to_owned()));
}

#[test]
fn test_add_cycle_cycle_name_empty_string() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_name": ""
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.add_new_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CYCLE_NAME_NULL.to_owned()));
}

#[test]
fn test_add_cycle_stages_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1,
    "cycle_name": "New Cycle Added"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.add_new_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_add_cycle_pipeline_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500,
    "cycle_name": "New Cycle Added"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.add_new_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

// Pipeline 0 is difficult to test; because we never want that happens. 
// We'll see that later when we have delete_cycle. 

#[test]
fn test_edit_cycle_ok() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 0,
    "cycle_name": "Edited Cycle Name"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_cycle(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1][0].clone();
  let old_ooi = old_serde["pipelines"][0][1][0].clone();
  assert_ne!(ooi["name"], old_ooi["name"]);
  assert_eq!(ooi["name"], "Edited Cycle Name");
}

#[test]
fn test_edit_cycle_name_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CYCLE_NAME_NULL.to_owned()));
}

#[test]
fn test_edit_cycle_name_empty_string() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 0,
    "cycle_name": ""
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CYCLE_NAME_NULL.to_owned()));
}

#[test]
fn test_edit_cycle_stages_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1,
    "cycle_index": 0,
    "cycle_name": "Edited Cycle Name"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_edit_cycle_pipeline_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500,
    "cycle_index": 0,
    "cycle_name": "Edited Cycle Name"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_edit_cycle_cycle_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 500,
    "cycle_name": "Edited Cycle Name"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_CYCLE_IDX.to_owned()));
}

#[test]
fn test_delete_cycle_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_cycle(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1].clone();
  let old_ooi = old_serde["pipelines"][0][1].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() - 1);
  let ooi2 = ooi[0].clone();
  let old_ooi2 = old_ooi[1].clone();
  assert_eq!(ooi2, old_ooi2);
}

#[test]
fn test_delete_cycle_cannot_only_one_left() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == CYCLE_AT_LEAST_ONE.to_owned()));
}

#[test]
fn test_delete_cycle_cycle_index_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == CYCLE_IDX_CANNOT_NULL.to_owned()));
}

#[test]
fn test_delete_cycle_stage_index_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_delete_cycle_pipeline_index_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500,
    "cycle_index": 0
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_delete_cycle_cycle_index_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 500
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_cycle(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_CYCLE_IDX.to_owned()));
}

#[test]
fn test_clear_cycle_ok() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.clear_cycle(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1].clone();
  let old_ooi = old_serde["pipelines"][0][1].clone();
  assert_eq!(ooi.as_array().unwrap().len(), 1);
  assert_ne!(old_ooi.as_array().unwrap().len(), 1);
  assert_ne!(ooi[0]["data"], old_ooi[0]["data"]);
  assert_eq!(ooi[0]["data"], json!([""]));
}

#[test]
fn test_clear_cycle_stages_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.clear_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_clear_cycle_pipeline_oob() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.clear_cycle(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_add_new_cycle_with_extra_clear_out() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 3,
    "cycle_name": "Cycle 2"
  }"#;
  let submit: SubmitResponse = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.add_new_cycle(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][3][1].clone();  // cycle index 1, since add. 
  let old_ooi = old_serde["pipelines"][0][3][0].clone();  // cycle index 0. 
  assert_eq!(ooi["data"], json!(["", ""]));
  assert!(!old_ooi["extra"].is_null());
  assert!(ooi["extra"].is_null());
}