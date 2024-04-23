use serde_json::{json, Value};

use crate::{checklist_dto::{SubmitChecklist, SubmitRespChecklist}, messages::{CHECKLIST_LEN_2, CHECKLIST_NONE, CHECKLIST_STRVEC, CL_EXTRA_LEN_NOT_MATCH, OOB_PIPELINE_IDX, OOB_STAGE_IDX, PIPELINE_IDX_CANNOT_NULL, TITLE_NONE, VEC_BOOL_ONLY}, reminder_dto::ReminderTrait, response_dto::ResponseTrait, CHECKLIST_TYPE, REMINDER_TYPE};

fn get_old_serde() -> Value {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
      {"name": "Stage 1", "pipeline": [
        {"ty": 2, "title": "Title 1", "others": "array/whatever as it is"},
        {"ty": 0, "title": "Title 2", "others": "array/whatever as it is"}
      ]},
      {"name": "Stage 2", "pipeline": [

      ]}
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

fn get_proj_serde() -> Value {
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
            "data": [true, false, true],
            "extra": [
              ["Question 1", "Question 2"], 
              [true, false]
            ]
          }
        ]
      ]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// ============================================
#[test]
fn test_new_checklist_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() + 1);
  assert!(ooi[0]["others"].is_string());
  assert!(ooi[2]["others"].is_array());
}

#[test]
fn test_new_checklist_title_cannot_be_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == TITLE_NONE.to_owned()));
}

#[test]
fn test_new_checklist_checklist_cannot_be_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Checklist Title"
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CHECKLIST_NONE.to_owned()));
}

#[test]
#[should_panic]
fn test_new_checklist_checklist_must_be_vec_string() {
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Checklist Title",
    "checklist": { "name": "some random value" }
  }"#;
  let _: SubmitChecklist = serde_json::from_str(&d).unwrap();
}

#[test]
fn test_new_checklist_stage_index_invalid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_edit_checklist_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"][1].clone();
  assert_eq!(ooi["title"], "Checklist Title");
  assert!(ooi["others"].is_array());
  let ooi2 = edited_serde["stages"][0]["pipeline"][0].clone();
  assert_eq!(ooi2["title"], "Title 1");
  assert!(ooi2["others"].is_string());
}

#[test]
fn test_edit_checklist_title_cannot_be_none() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == TITLE_NONE.to_owned()));
}

#[test]
fn test_edit_checklist_checklist_cannot_be_none() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "title": "Checklist Title"
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CHECKLIST_NONE.to_owned()));
}

#[test]
fn test_edit_checklist_invalid_stage_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_edit_checklist_invalid_pipeline_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_edit_checklist_pipeline_index_absent_cannot() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == PIPELINE_IDX_CANNOT_NULL.to_owned()));
}

// This is a feature, not a bug.
#[test]
fn test_edit_forced_ty_to_be_new() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "title": "Checklist Title",
    "checklist": ["question 1", "question 2"]
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"][1].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"][1].clone();
  assert_eq!(old_ooi["ty"], REMINDER_TYPE);
  assert_eq!(ooi["ty"], CHECKLIST_TYPE);
}

#[test]
fn test_delete_checklist_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() - 1);
  assert_eq!(ooi[0]["title"], "Title 2");
}

#[test]
fn test_delete_checklist_invalid_stage_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_delete_checklist_invalid_pipeline_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 5200
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_delete_checklist_pipeline_index_absent_cannot() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0
  }"#;
  let submit: SubmitChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == PIPELINE_IDX_CANNOT_NULL.to_owned()));
}

// =======================================================
#[test]
fn test_edit_response_valid() {
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      ["Question 1", "Question 2"], 
      [true, false]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][0][0].clone();
  let old_ooi = old_serde["pipelines"][0][0][0].clone();
  assert_ne!(ooi["data"], old_ooi["data"]);
  assert_eq!(ooi["data"], json!([true, false, true]));
  assert!(old_ooi["extra"].is_null());
  assert!(!ooi["extra"].is_null());
  assert_eq!(ooi["extra"], json!([["Question 1", "Question 2"], [true, false]]));
}

#[test]
fn test_edit_response_valid_2() {
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][0][0].clone();
  let old_ooi = old_serde["pipelines"][0][0][0].clone();
  assert_ne!(ooi["data"], old_ooi["data"]);
  assert_eq!(ooi["data"], json!([true, false, true]));
  assert!(ooi["extra"].is_null());
}

#[test]
fn test_edit_response_not_boolean() {
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, "some string"]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == VEC_BOOL_ONLY.to_owned()));
}

// Extra must have length = 2. 
// First array is purely string, for questions. 
// Second array is purely boolean, for true/false. 
// First array and second array must have same length. 
#[test]
fn test_extra_len_more_than_two_failed() {
  // Len > 2
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      ["Question 1", "Question 2"], 
      [true, false],
      ["Longer len", "not used"]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();
  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CHECKLIST_LEN_2.to_owned()));
}

#[test]
fn test_extra_len_less_than_two_failed() {
  // Len < 2
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      ["Question 1", "Question 2"]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();
  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CHECKLIST_LEN_2.to_owned()));
}

#[test]
fn test_first_array_not_pure_string_failed() {
  // First not pure string. 
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      [true, "Question 2"], 
      [true, false]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();
  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CHECKLIST_STRVEC.to_owned()));
}

#[test]
fn test_second_array_not_pure_bool_failed() {
  // Second not pure vec
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      ["Question 1", "Question 2"], 
      [true, "this is not boolean"]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();
  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == VEC_BOOL_ONLY.to_owned()));
}

#[test]
fn test_extra_question_answer_not_equal_length_failed() {
  // And not equal length
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      ["Question 1", "Question 2"], 
      [true, false, true]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();
  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CL_EXTRA_LEN_NOT_MATCH.to_owned()));

  // The other way round
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "cycle_index": 0,
    "checklist": [true, false, true],
    "extra_checklist": [
      ["Question 1", "Question 2", "Question 3"], 
      [true, false]
    ]
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();
  let edited_serde = submit.edit_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == CL_EXTRA_LEN_NOT_MATCH.to_owned()));
}

#[test]
fn test_delete_response_reset_all_to_expected() {
  let old_serde = get_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 0
  }"#;
  let submit: SubmitRespChecklist = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1][0].clone();
  let old_ooi = old_serde["pipelines"][0][1][0].clone();
  assert!(!old_ooi["extra"].is_null());
  assert!(ooi["extra"].is_null());
  assert_ne!(ooi["data"], old_ooi["data"]);
  assert_eq!(old_ooi["data"].as_array().unwrap().len(), 3, "old_ooi array len not 3, you changed proj_serde?");
  assert_eq!(ooi["data"], json!([false, false, false]));
}

