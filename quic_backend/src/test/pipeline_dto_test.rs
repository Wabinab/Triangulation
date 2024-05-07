use serde_json::Value;

use crate::{messages::{OOB_CYCLE_IDX, OOB_PIPELINE_IDX, OOB_STAGE_IDX}, pipeline_dto::{gen_empty_pipeline, PipelineTrait, SubmitPipeline}};

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
      [{
        "name": "0",
        "data": [0, 4, [1, 0]]
      }],
      [{
        "name": "0",
        "data": ["Here are some answers from the paragraph"]
      }]
    ], 
    []
  ]"#;

  // But that's not what we want, what we want is this: 
  let c = r#"[
    [
      [{
        "name": "0",
        "data": ["", "", ""]
      }],
      [{
        "name": "0",
        "data": [""]
      }]
    ],
    []
  ]"#;
  serde_json::from_str(&c).unwrap()
}

fn get_old_serde_proj() -> Value {
  let c = r#" {
    "name": "With Special Question",
    "description": "This is version 3 of template, with special questions. ",
    "t_uuid": "018e1be4-582c-70bf-8972-d0e5c4786f2a",
    "uuid": "018e5065-393a-7cc9-beca-9945f961068f",
    "t_ver": 2,
    "pipelines": [
      [
        [{"name": "0", "data": ["", "", ""]}],
        [{"name": "0", "data": ["mid response"]}, 
          {"name": "1", "data": ["mid response 2"]}
        ],
        [{"name": "0", "data": [
          "short response",
          "long response",
          1,
          [0, 1],
          3,
          [3, 4, 0],
          [[1, 2], [2]],
          "2024-03-20T11:23"
        ]}],
        [{"name": "0", "data": [true, false, true], "extra": [["Question 1"], [true]]},
          {"name": "1", "data": [false, false, true], "extra": null}
        ]
      ]
    ]
  }"#;
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
  let empty_pipeline = gen_empty_pipeline(old_serde.clone());
  assert_eq!(empty_pipeline, get_specific_pipeline());
}

#[test]
fn test_response_valid() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 0
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_response(old_serde).unwrap();
  assert_eq!(edited_serde.as_array().unwrap().len(), 1);
  assert_eq!(edited_serde[0], "mid response");
}

#[test]
fn test_response_valid_without_cycle() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_response(old_serde).unwrap();
  assert_eq!(edited_serde.as_array().unwrap().len(), 2);
  assert_eq!(edited_serde[1]["data"][0], "mid response 2");
}

#[test]
fn test_response_invalid_stage_index() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1,
    "cycle_index": 0
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();
  
  let edited_serde = submit.get_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_response_invalid_pipeline_index() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500,
    "cycle_index": 0
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();
  
  let edited_serde = submit.get_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_response_invalid_cycle_index() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "cycle_index": 500
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_response(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_CYCLE_IDX.to_owned()));
}

#[test]
fn test_get_response_checklist_have_extra() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 3,
    "cycle_index": 0
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();
  
  let edited_serde = submit.get_response_checklist(old_serde.clone()).unwrap();
  let ooi = edited_serde.clone();
  let old_ooi = old_serde["pipelines"][0][3][0].clone();
  assert!(!old_ooi["extra"].is_null());
  assert_eq!(ooi["extra"], old_ooi["extra"]);
}

#[test]
fn test_get_normal_response_for_checklist_no_extra() {
  let old_serde = get_old_serde_proj();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 3,
    "cycle_index": 0
  }"#;
  let submit: SubmitPipeline = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.get_response(old_serde.clone()).unwrap();
  let ooi = edited_serde.clone();
  let old_ooi = old_serde["pipelines"][0][3][0].clone();
  assert!(!old_ooi["extra"].is_null());
  assert!(ooi["extra"].is_null());
}