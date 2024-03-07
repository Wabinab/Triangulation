use serde_json::Value;

use crate::template_dto::to_nlist;

use super::*;

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

// =====================================================

#[test]
fn test_edit_template() {

}

#[test]
fn test_deserialize_nlist() {
  let old_serde = get_old_serde();
  let nlist = to_nlist(old_serde.clone());

  assert_eq!(nlist.name, old_serde["name"]);
  assert_eq!(nlist.uuid, old_serde["uuid"]);
  assert_eq!(nlist.description, old_serde["description"]);

  let stages = old_serde["stages"].as_array().unwrap().clone();
  assert_eq!(nlist.stages.len(), stages.len());
  assert_eq!(nlist.stages[0].name, stages[0]["name"]);
  assert_eq!(nlist.stages[1].name, stages[1]["name"]);

  let pipeline = stages[0]["pipeline"].as_array().unwrap().clone();
  let npipeline = nlist.stages[0].pipeline.clone();
  assert_eq!(npipeline.len(), pipeline.len());
  assert_eq!(npipeline[0].ty, pipeline[0]["ty"]);
  assert_eq!(npipeline[1].ty, pipeline[1]["ty"]);
  assert_eq!(npipeline[0].title, pipeline[0]["title"]);
  assert_eq!(npipeline[1].title, pipeline[1]["title"]);
}