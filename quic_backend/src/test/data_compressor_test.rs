use std::path::PathBuf;

use serde_json::{json, Value};
use uuid::Uuid;
use crate::compressor::{compress_and_save_fullpath, retrieve_decompress_fullpath};

use super::helper::{get_datapath, cleanup};

fn gen_testfile() -> String {
  let mut filename = Uuid::new_v4().to_string();
  filename.push_str("_compressor.json.zl");
  return filename;
}

fn get_filepath() -> PathBuf {
  let filename = gen_testfile();
  let mut filepath = get_datapath();
  filepath.push(filename);
  filepath
}

fn repeated_test(data: Value, filepath: PathBuf) {
  let ret = compress_and_save_fullpath(data.clone().to_string(), filepath.clone());
  assert!(ret.is_ok());
  let ret2 = retrieve_decompress_fullpath(filepath.clone());
  assert!(ret2.is_ok());
  assert_eq!(ret2.unwrap(), data.clone());
  cleanup(filepath)
}


// ==========================================
#[test]
fn test_json_empty_curly_braces() {
  let data = json!({});  
  repeated_test(data, get_filepath());
}

#[test]
fn test_json_empty_array() {
  let data = json!([]);
  repeated_test(data, get_filepath());
}

#[test]
fn test_json_1() {
  let data = json!({
    "name": "arbitrary",
    "layer": 3,
    "production": 0.8,
    "liver": {
      "second_layer": [
        "third_layer_1", 2, 3.015
      ],
      "second_layer_2": "what?"
    }
  });
  repeated_test(data, get_filepath());
}

#[test]
fn test_json_2() {
  let data = json!([
    "index_0",
    ["index_1", 2, 3.015],
    2,
    { "name": "redundant", "others": ["meh", 213], "goat": 2}
  ]);
    repeated_test(data, get_filepath());
}