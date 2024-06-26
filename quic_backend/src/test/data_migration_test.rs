use serde_json::{json, Value};

use crate::{messages::OOB_MIGRATE, migration::{migrate_data, unsafe_migrate_data}};

/// This is so extensive that we need a single test to ensure its
/// robustness. 

fn get_templ_serde() -> Value {
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

// // This is the original data. 
// // We can see this is 3, 1. 
// let _ = r#"[
//     [
//       [0, 4, [1, 0]],
//       ["Here are some answers from the paragraph"]
//     ], 
//     [
//        ["empty"]
//     ]
//   ]"#;

// =======================================
#[test]
fn test_less_data_pipeline_level() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone()).unwrap();

  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, ""]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "safe failed");

  // Test unsafe too! 
  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, ""]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_more_data_pipeline_level() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3, "this cause oob"]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone());
  assert!(submit.is_err_and(|x| x == OOB_MIGRATE.to_owned()), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_more_data_pipeline_level_empty() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3, ""]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone()).unwrap();

  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_less_data_question_level() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone()).unwrap();

  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": [""]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": [""]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_more_data_question_level() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
      [{"name": "0", "data": ["this is extra"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone());
  assert!(submit.is_err_and(|x| x == OOB_MIGRATE.to_owned()), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_more_data_question_level_empty() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
      [{"name": "0", "data": [""]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone()).unwrap();
  
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_less_data_stage_level() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone()).unwrap();

  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
    ],
    [
      [{"name": "0", "data": [""]}]
    ]
  ]), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
    ],
    [
      [{"name": "0", "data": [""]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_more_data_stage_level() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
      [{"name": "0", "data": [""]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ],
    [
      [{"name": "0", "data": ["new pipeline causes error"]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone());
  assert!(submit.is_err_and(|x| x == OOB_MIGRATE.to_owned()), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}

#[test]
fn test_more_data_stage_level_empty() {
  let c = r#"[
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
      [{"name": "0", "data": [""]}]
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ],
    [
      [{"name": "0", "data": [""]}]
    ]
  ]"#;
  let d: Value = serde_json::from_str(&c).unwrap();
  let submit = migrate_data(get_templ_serde(), d.clone()).unwrap();

  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "safe failed");

  let submit = unsafe_migrate_data(get_templ_serde(), d).unwrap();
  assert_eq!(submit, json!([
    [
      [{"name": "0", "data": ["not empty", 0, 3]}],
      [{"name": "0", "data": ["blah"]}],
    ],
    [
      [{"name": "0", "data": ["something"]}]
    ]
  ]), "unsafe failed");
}