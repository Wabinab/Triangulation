use serde_json::Value;

use crate::project_dto::{ProjectTrait, SubmitProject};

fn get_old_serde() -> Value {
  let c = r#"{
    "name": "...",
    "uuid": "This is project uuid",
    "description": "...",
    "t_uuid": "Template UUID",
    "t_ver": 0,
    "pipelines": [
      [
        ["answer 1", "answer 2", 3, ["grid answer 4.1", "grid answer 4.2"]],
        ["pipeline 2 answers here"]
      ],
      ["stages 2 answers here"]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// ================================================

#[test]
fn test_new_project() {
  let d = r#"{
    "name": "New Project",
    "description": "Project description",
    "template_uuid": "Some_uuid"
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let uuid = "Test UUID".to_string();
  let edited_serde = submit.new_project(uuid.clone(), 0).unwrap();
  assert_eq!(edited_serde["name"], "New Project");
  assert_eq!(edited_serde["uuid"], uuid.as_str());
  assert_eq!(edited_serde["description"], "Project description");
  assert_eq!(edited_serde["t_uuid"], "Some_uuid");
  assert_eq!(edited_serde["t_ver"], 0);
}

#[test]
fn test_new_project_uuid_null() {
  let d = r#"{
    "name": "New Project",
    "description": "Project description"
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let uuid = "Test UUID".to_string();
  let edited_serde = submit.new_project(uuid.clone(), 0);
  assert!(edited_serde.is_err_and(|x| x == "Template must not be null.".to_owned()));
}

#[test]
fn test_edit_project() {
  let old_serde = get_old_serde();

  let d = r#"{
    "name": "New Project",
    "description": "Project description"
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_template(old_serde.clone()).unwrap();
  assert_ne!(edited_serde["name"], old_serde["name"]);
  assert_eq!(edited_serde["name"], "New Project");
  assert_ne!(edited_serde["description"], old_serde["description"]);
  assert_eq!(edited_serde["description"], "Project description");
  assert_eq!(edited_serde["t_ver"], old_serde["t_ver"]);
}

#[test]
fn test_edit_project_with_ver() {
  let old_serde = get_old_serde();

  let d = r#"{
    "name": "New Project",
    "description": "Project description",
    "version": 13
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_template(old_serde.clone()).unwrap();
  assert_eq!(edited_serde["name"], "New Project");
  assert_eq!(edited_serde["description"], "Project description");
  assert_ne!(edited_serde["t_ver"], old_serde["t_ver"]);
  assert_eq!(edited_serde["t_ver"], 13);
}