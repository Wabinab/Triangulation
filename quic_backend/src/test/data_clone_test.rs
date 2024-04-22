use crate::{clone::{clone_project, clone_template}, compressor::retrieve_decompress_fullpath, file::modify_datapath, *};
use uuid::Uuid;

use self::compressor::compress_and_save_fullpath;

use super::helper::{get_datapath, cleanup};

fn gen_testfile() -> String {
  let mut filename = Uuid::new_v4().to_string();
  filename.push_str("_compressor.json.zl");
  return filename;
}

fn get_filepath(filename: String, directory: String) -> PathBuf {
  // let filename = gen_testfile();
  let mut filepath = get_datapath();
  filepath.push(directory);
  filepath.push(filename);
  filepath
}

fn get_template_serde() -> Value {
  let c = r#"{
    "name": "First Template",
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

fn get_project_serde() -> Value {
  let c = r#"{
    "name": "Some Name",
    "uuid": "This is project uuid",
    "description": "Some Desc",
    "t_uuid": "Template UUID",
    "t_ver": 0,
    "pipelines": [
      [
        [{"name": "0", "data": ["answer 1", "answer 2", 3, ["grid answer 4.1", "grid answer 4.2"]]}],
        [{"name": "0", "data": ["pipeline 2 answers here"]}]
      ],
      [
        [{"name": "0", "data": ["stages 2 answers here"]}]
      ]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// ==================================================
#[test]
fn test_clone_template_works_as_expected() {
  let directory = "template";
  let filename = gen_testfile();
  let filepath = get_filepath(filename.clone(), directory.to_owned());
  let data = get_template_serde();

  let ret = compress_and_save_fullpath(data.clone().to_string(), filepath.clone());
  assert!(ret.is_ok());
  let ret2 = retrieve_decompress_fullpath(filepath.clone());
  assert!(ret2.is_ok());
  
  let retval = clone_template(get_datapath(), filename.clone());
  assert!(retval.is_ok());
  let mut filepath2 = modify_datapath(get_datapath(), directory);
  filepath2.push(retval.unwrap()["filename"].as_str().unwrap());
  let ret3 = retrieve_decompress_fullpath(filepath2.clone());
  assert!(ret3.is_ok());

  let ooi = ret3.unwrap().clone();
  let old_ooi = ret2.unwrap().clone();
  assert_ne!(ooi["name"], old_ooi["name"]);
  assert_ne!(ooi["uuid"], old_ooi["uuid"]);
  assert!(ooi["name"].as_str().unwrap().to_owned().contains("(Copy)"));
  assert_eq!(ooi["description"], old_ooi["description"]);
  assert_eq!(ooi["stages"], old_ooi["stages"]);

  cleanup(filepath);
  cleanup(filepath2);
}

#[test]
fn test_clone_project_works_as_expected() {
  let directory = "project";
  let filename = gen_testfile();
  let filepath = get_filepath(filename.clone(), directory.to_owned());
  let data = get_project_serde();

  let ret = compress_and_save_fullpath(data.clone().to_string(), filepath.clone());
  assert!(ret.is_ok());
  let ret2 = retrieve_decompress_fullpath(filepath.clone());
  assert!(ret2.is_ok());

  let retval = clone_project(get_datapath(), filename.clone());
  assert!(retval.is_ok());
  let mut filepath2 = modify_datapath(get_datapath(), directory);
  filepath2.push(retval.unwrap()["filename"].as_str().unwrap());
  let ret3 = retrieve_decompress_fullpath(filepath2.clone());
  assert!(ret3.is_ok());

  let ooi = ret3.unwrap().clone();
  let old_ooi = ret2.unwrap().clone();
  assert_ne!(ooi["name"], old_ooi["name"]);
  assert_ne!(ooi["uuid"], old_ooi["uuid"]);
  assert!(ooi["name"].as_str().unwrap().to_owned().contains("(Copy)"));
  assert_eq!(ooi["description"], old_ooi["description"]);
  assert_eq!(ooi["t_uuid"], old_ooi["t_uuid"]);
  assert_eq!(ooi["t_ver"], old_ooi["t_ver"]);
  assert_eq!(ooi["pipelines"], old_ooi["pipelines"]);

  cleanup(filepath);
  cleanup(filepath2);
}