use serde_json::Value;

use crate::{checklist_dto::SubmitChecklist, messages::{CHECKLIST_NONE, OOB_PIPELINE_IDX, OOB_STAGE_IDX, PIPELINE_IDX_CANNOT_NULL, TITLE_NONE}, reminder_dto::ReminderTrait, CHECKLIST_TYPE, REMINDER_TYPE};

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