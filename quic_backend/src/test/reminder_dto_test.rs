use serde_json::Value;

use crate::{messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX, QUESTION_NONE, REMINDER_IDX_CANNOT_NULL, TITLE_NONE}, reminder_dto::{ReminderTrait, SubmitReminder}};

// use super::*;

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

// ====================================================================
#[test]
fn test_new_reminder_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "New Title",
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() + 1);
  assert!(ooi[0]["others"].is_string());
  assert!(ooi[2]["others"].is_array());
}

#[test]
fn test_new_reminder_title_cannot_be_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == TITLE_NONE.to_owned()));
}

#[test]
fn test_new_reminder_question_cannot_be_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "New Title"
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == QUESTION_NONE.to_owned()));
}

#[test]
fn test_new_reminder_stage_index_invalid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "title": "New Title",
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_edit_reminder_valid() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "reminder_index": 1,
    "title": "New Title",
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"][1].clone();
  assert_eq!(ooi["title"], "New Title");
  assert!(ooi["others"].is_array());
  let ooi2 = edited_serde["stages"][0]["pipeline"][0].clone();
  assert_eq!(ooi2["title"], "Title 1");
  assert!(ooi2["others"].is_string());
}

#[test]
fn test_edit_reminder_title_cannot_be_none() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "reminder_index": 1,
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == TITLE_NONE.to_owned()));
}

#[test]
fn test_edit_reminder_question_cannot_be_none() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "reminder_index": 1,
    "title": "New Title"
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == QUESTION_NONE.to_owned()));
}

#[test]
fn test_edit_reminder_invalid_stage_index() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 500, 
    "reminder_index": 1,
    "title": "New Title",
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}


#[test]
fn test_edit_reminder_invalid_reminder_index() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0, 
    "reminder_index": 500,
    "title": "New Title",
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_edit_reminder_index_null() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "New Title",
    "question": ["question 1", "question 2"]
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == REMINDER_IDX_CANNOT_NULL.to_owned()));
}


#[test]
fn test_delete_reminder() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "reminder_index": 0
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() - 1);
  assert_eq!(ooi[0]["title"], "Title 2");
}

#[test]
fn test_delete_reminder_invalid_stage_index() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 500, 
    "reminder_index": 1
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_delete_reminder_invalid_reminder_index() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0, 
    "reminder_index": 500
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_delete_reminder_index_null() {
  let old_serde = get_old_serde();

  let d = r#"{
    "filename": "...",
    "stage_index": 0
  }"#;
  let submit: SubmitReminder = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == REMINDER_IDX_CANNOT_NULL.to_owned()));
}