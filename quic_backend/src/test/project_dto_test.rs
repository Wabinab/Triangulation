use serde_json::{json, Value};

use crate::{messages::{TEMPLATE_CANNOT_NULL, VER_TEMP_NONE}, project_dto::{to_nlist_proj, ProjectTrait, SubmitProject}};

fn get_old_serde() -> Value {
  let c = r#"{
    "name": "Some Name",
    "uuid": "This is project uuid",
    "description": "Some Desc",
    "t_uuid": "Template UUID",
    "t_ver": 0,
    "pipelines": [
      [
        ["answer 1", "answer 2", 3, ["grid answer 4.1", "grid answer 4.2"]],
        ["pipeline 2 answers here"]
      ],
      [
        ["stages 2 answers here"]
      ]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

fn get_aligning_templ_serde() -> Value {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {
            "ty": 0,
            "others": [
              {
                "t": "2",
                "q": "Question 1",
                "r": [
                  "Option 1 is here",
                  "We now have option 2"
                ]
              },
              {
                "min": 1,
                "max_name": "Good",
                "q": "This is a range",
                "min_name": "Bad",
                "t": "4",
                "max": 5
              },
              {
                "q": "This is a grid",
                "c": [
                  "Nothing we can do",
                  "Are you sure?"
                ],
                "r": [
                  "What can we do?",
                  "You think so? "
                ],
                "t": "5"
              },
              {
                "q": "With only a single paragraph",
                "t": "1"
              }
            ],
            "title": "This is the title"
          },
          {
            "title": "Another question",
            "ty": 0,
            "others": [
              {
                "q": "With only a single paragraph",
                "t": "1"
              }
            ]
          }
        ]},
        {"name": "Stage 2", "pipeline": [
          {
            "title": "Another question at another pipeline",
            "ty": 0,
            "others": [
              {
                "q": "With only a single paragraph",
                "t": "1"
              }
            ]
          }
        ]}
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

fn get_template_serde() -> Value {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
        {"name": "Stage 1", "pipeline": [
          {"ty": 0, "title": "Title 1", "others": ["array/whatever as it is"]},
          {"ty": 0, "title": "Title 2", "others": ["array/whatever as it is"]}
        ]},
        {"name": "Stage 2", "pipeline": [

        ]}
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
  let edited_serde = submit.new_project(uuid.clone(), 0, get_template_serde());
  assert!(edited_serde.is_ok());
  let edited_serde = edited_serde.unwrap();
  assert_eq!(edited_serde["name"], "New Project");
  assert_eq!(edited_serde["uuid"], uuid.as_str());
  assert_eq!(edited_serde["description"], "Project description");
  assert_eq!(edited_serde["t_uuid"], "Some_uuid");
  assert_eq!(edited_serde["t_ver"], 0);
  // assert_eq!(edited_serde["pipelines"], json!([]));
}

#[test]
fn test_new_project_uuid_null() {
  let d = r#"{
    "name": "New Project",
    "description": "Project description"
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let uuid = "Test UUID".to_string();
  let edited_serde = submit.new_project(uuid.clone(), 0, get_template_serde());
  assert!(edited_serde.is_err_and(|x| x == TEMPLATE_CANNOT_NULL.to_owned()));
}

#[test]
fn test_edit_project() {
  let old_serde = get_old_serde();

  let d = r#"{
    "name": "New Project",
    "description": "Project description"
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_project(old_serde.clone(), None).unwrap();
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

  let temp = get_aligning_templ_serde();

  let edited_serde = submit.edit_project(old_serde.clone(), Some(temp)).unwrap();
  assert_eq!(edited_serde["name"], "New Project");
  assert_eq!(edited_serde["description"], "Project description");
  assert_ne!(edited_serde["t_ver"], old_serde["t_ver"]);
  assert_eq!(edited_serde["t_ver"], 13);
}

#[test]
fn test_edit_project_with_ver_no_file() {
  let old_serde = get_old_serde();

  let d = r#"{
    "name": "New Project",
    "description": "Project description",
    "version": 13
  }"#;
  let submit: SubmitProject = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_project(old_serde.clone(), None);
  assert!(edited_serde.is_err_and(|x| x == VER_TEMP_NONE.to_owned()));
}

#[test]
fn test_to_proj_nlist() {
  let old_serde = get_old_serde();
  let data = to_nlist_proj(old_serde.clone());
  assert_eq!(data.name, old_serde["name"]);
  assert_eq!(data.description, old_serde["description"]);
  assert_eq!(data.uuid, old_serde["uuid"]);
  assert_eq!(data.t_uuid, old_serde["t_uuid"]);
  assert_eq!(data.t_ver, old_serde["t_ver"]);
}