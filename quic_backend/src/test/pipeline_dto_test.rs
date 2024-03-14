use serde_json::Value;

use crate::{messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX}, pipeline_dto::{gen_empty_pipeline, PipelineTrait, SubmitPipeline}};

fn get_old_serde() -> Value {
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

        ]}
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

fn get_specific_pipeline() -> Value {
  // The outermost [] isn't included. Anything second layer is stages, anything 3rd layer is pipeline. 
  // First is option, so we can index options choice. 
  // Second is range 1-5, so that's 4. 
  // Third is grid, 2x2, so row 1 maps column 2 (index 1), row 2 maps column 1 (index 0)
  // Second pipeline is a paragraph question
  // Second stage have nothing, so empty. 
  let _ = r#"[
    [
      [0, 4, [1, 0]],
      ["Here are some answers from the paragraph"]
    ], 
    []
  ]"#;

  // But that's not what we want, what we want is this: 
  let c = r#"[
    [
      ["", "", ""],
      [""]
    ],
    []
  ]"#;
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
  assert_eq!(edited_serde["title"], "Another question");
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

#[test]
fn test_gen_empty_pipeline_correct() {
  let old_serde = get_old_serde();
  let empty_pipeline = gen_empty_pipeline(old_serde.clone()).unwrap();
  assert_eq!(empty_pipeline, get_specific_pipeline());
}