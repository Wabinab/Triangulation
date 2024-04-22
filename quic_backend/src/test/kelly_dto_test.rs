use serde_json::Value;

use crate::{kelly_dto::SubmitKelly, messages::{OOB_PIPELINE_IDX, OOB_STAGE_IDX, PIPELINE_IDX_CANNOT_NULL, TITLE_NONE, TRANSACTION_NONE}, reminder_dto::ReminderTrait, response_dto::ResponseTrait};

fn get_old_serde() -> Value {
  let c = r#"{
    "name": "...",
    "uuid": "...",
    "description": "...",
    "stages": [
      {"name": "Stage 1", "pipeline": [
        {"ty": 1, "title": "Title 1", "others": "array of transactions"},
        {"ty": 1, "title": "Title 2", "others": "array of transactions"}
      ]},
      {"name": "Stage 2", "pipeline": [

      ]}
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// Kelly DTO doesn't have "name" and "data". 
fn get_old_proj_serde() -> Value {
  let c = r#"{
    "name": "First Kelly Project",
    "description": "...",
    "t_uuid": "...",
    "uuid": "...",
    "t_ver": 2,
    "pipelines": [
      [
        [
          {
            "coin": "NEAR",
            "buy": false,
            "price": "6",
            "amt": "25",
            "price_1": null,
            "amt_1": null,
            "pred_prob": "1"
          },
          {
            "coin": "BTC",
            "buy": true,
            "price": "35",
            "amt": "10",
            "price_1": "0.2",
            "amt_1": "10",
            "pred_prob": "1"
          }
        ], 
        [
          {
            "coin": "BCH",
            "buy": true,
            "price": "21",
            "amt": "10",
            "price_1": "80",
            "amt_1": "10",
            "pred_prob": "1"
          }
        ]
      ]
    ]
  }"#;
  serde_json::from_str(&c).unwrap()
}

// ========================================================
#[test]
fn test_new_kelly_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() + 1);
  assert!(ooi[2]["others"].is_array(), "{:#?}", ooi[2]["others"]);
  // let obj = ooi[2]["others"][0].clone();
  // assert_eq!(obj["coin"], "BTC");
  // assert_eq!(obj["buy"], true);
  // assert_eq!(obj["price"], "20");
  // assert_eq!(obj["amt"], "10");
  // assert!(obj["price_1"].is_null());
  // assert!(obj["amt_1"].is_null());
  // assert_eq!(obj["pred_prob"], "1");
}

#[test]
fn test_new_kelly_title_cannot_be_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == TITLE_NONE.to_owned()));
}

// #[test]
// fn test_new_kelly_transactions_cannot_be_null() {
//   let old_serde = get_old_serde();
//   let d = r#"{
//     "filename": "...",
//     "stage_index": 0,
//     "title": "Kelly Transactions"
//   }"#;
//   let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

//   let edited_serde = submit.new_reminder(old_serde);
//   assert!(edited_serde.is_err_and(|x| x == TRANSACTION_NONE.to_owned()));
// }

#[test]
#[should_panic]
fn test_kelly_transaction_invalid() {
  // let old_serde = get_old_serde();
  // We removed pred_prob. 
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10"
      }
    ]
  }"#;
  let _: SubmitKelly = serde_json::from_str(&d).unwrap();
}

#[test]
fn test_new_kelly_stage_index_invalid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.new_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_edit_kelly_valid() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"][1].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"][1].clone();
  assert_ne!(ooi["title"], old_ooi["title"]);
  assert_eq!(ooi["title"], "Kelly Transactions");
  assert_eq!(ooi["others"], old_ooi["others"]); // no change during template. 
  let ooi2 = edited_serde["stages"][0]["pipeline"][0].clone();
  assert_eq!(ooi2["title"], "Title 1");
}

#[test]
fn test_edit_kelly_title_cannot_be_none() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == TITLE_NONE.to_owned()));
}

// #[test]
// fn test_edit_kelly_transaction_cannot_be_none() {
//   let old_serde = get_old_serde();
//   let d = r#"{
//     "filename": "...",
//     "stage_index": 0,
//     "pipeline_index": 1,
//     "title": "Kelly Transactions"
//   }"#;
//   let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

//   let edited_serde = submit.edit_reminder(old_serde);
//   assert!(edited_serde.is_err_and(|x| x == TRANSACTION_NONE.to_owned()));
// }

#[test]
fn test_edit_kelly_invalid_stage_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 1,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_edit_kelly_invalid_pipeline_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_edit_pipeline_index_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "title": "Kelly Transactions",
    "transactions": [
      {
        "coin": "BTC",
        "buy": true,
        "price": "20",
        "amt": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == PIPELINE_IDX_CANNOT_NULL.to_owned()));
}

#[test]
fn test_delete_kelly() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde.clone()).unwrap();
  let ooi = edited_serde["stages"][0]["pipeline"].clone();
  let old_ooi = old_serde["stages"][0]["pipeline"].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() - 1);
  assert_eq!(ooi[0]["title"], "Title 2");
}

#[test]
fn test_delete_kelly_invalid_stage_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500,
    "pipeline_index": 0
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_STAGE_IDX.to_owned()));
}

#[test]
fn test_delete_kelly_invalid_pipeline_index() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}

#[test]
fn test_delete_pipeline_index_null() {
  let old_serde = get_old_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 500
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_reminder(old_serde);
  assert!(edited_serde.is_err_and(|x| x == PIPELINE_IDX_CANNOT_NULL.to_owned()));
}

// ===========================================================
// Test response
#[test]
fn test_edit_response_correct() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0,
    "transactions": [
      {
        "coin": "NEWCOIN",
        "buy": false,
        "price": "10",
        "amt": "27",
        "price_1": null,
        "amt_1": null,
        "pred_prob": "0.8"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][0].clone();
  let old_ooi = old_serde["pipelines"][0][0].clone();
  assert_ne!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len());
  assert_ne!(ooi[0]["coin"], old_ooi[0]["coin"]);
  assert_eq!(ooi[0]["buy"], old_ooi[0]["buy"]);
  let ooi2 = edited_serde["pipelines"][0][1].clone();
  assert_ne!(ooi2[0]["coin"], ooi[0]["coin"]);
}

// pipeline index 1 (pipeline 2) have only 1, now we make 2. 
// Keep the original unchanged.
#[test]
fn test_edit_add_new() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 1,
    "transactions": [
      {
        "coin": "NEWCOIN",
        "buy": false,
        "price": "10",
        "amt": "27",
        "price_1": null,
        "amt_1": null,
        "pred_prob": "0.8"
      },
      {
        "coin": "BCH",
        "buy": true,
        "price": "21",
        "amt": "10",
        "price_1": "80",
        "amt_1": "10",
        "pred_prob": "1"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][1].clone();
  let old_ooi = old_serde["pipelines"][0][1].clone();
  assert_eq!(ooi.as_array().unwrap().len(), old_ooi.as_array().unwrap().len() + 1);
  assert_ne!(ooi[0], old_ooi[0]);
  assert_eq!(ooi[1], old_ooi[0]);
  assert_eq!(ooi[0]["pred_prob"], "0.8");
}

#[test]
fn test_edit_response_pipeline_index_none() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "transactions": [
      {
        "coin": "NEWCOIN",
        "buy": false,
        "price": "10",
        "amt": "27",
        "price_1": null,
        "amt_1": null,
        "pred_prob": "0.8"
      }
    ]
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == PIPELINE_IDX_CANNOT_NULL.to_owned()));
}

#[test]
fn test_edit_response_transactions_none() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.edit_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == TRANSACTION_NONE.to_owned()));
}

// Skip stage index and pipeline index

#[test]
fn test_delete_existing() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 0
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone()).unwrap();
  let ooi = edited_serde["pipelines"][0][0].clone();
  let old_ooi = old_serde["pipelines"][0][0].clone();
  assert_eq!(ooi.as_array().unwrap().len(), 0);
  assert_eq!(old_ooi.as_array().unwrap().len(), 2);
}

#[test]
fn test_delete_response_pipeline_index_none() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == PIPELINE_IDX_CANNOT_NULL.to_owned()));
}

#[test]
fn test_delete_outside_pipeline() {
  let old_serde = get_old_proj_serde();
  let d = r#"{
    "filename": "...",
    "stage_index": 0,
    "pipeline_index": 500
  }"#;
  let submit: SubmitKelly = serde_json::from_str(&d).unwrap();

  let edited_serde = submit.delete_response(old_serde.clone());
  assert!(edited_serde.is_err_and(|x| x == OOB_PIPELINE_IDX.to_owned()));
}