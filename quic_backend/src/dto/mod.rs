/// DTO refers to Data Object, or Data To Object, whatever you call it. 
/// Basically, converts a json string (data)  to an object (struct). 

pub(crate) mod template_dto;
pub(crate) mod stage_dto;
pub(crate) mod pipeline_dto;
pub(crate) mod reminder_dto;
pub(crate) mod project_dto;
pub(crate) mod response_dto;
pub(crate) mod filelist_dto;
pub(crate) mod misc_dto;
pub(crate) mod kelly_dto;
pub(crate) mod checklist_dto;

// ================================================================
// NOTE: All these have problems, they don't live long enough. 
// use bytes::Bytes;
// use serde::de;

// /// Convert to JSON with specific DTO
// pub(crate) fn to_dto<T>(obj_str: &'static str) -> serde_json::Result<T>
// where
//     T: de::Deserialize<'static>, 
// {
//     let obj: T = serde_json::from_str(obj_str)?;
//     Ok(obj)
// }

// pub(crate) fn to_dto<T>(obj_bytes: Bytes) -> serde_json::Result<T>
// where
//     T: de::Deserialize<'static>, 
// {
//     return serde_json::from_slice(&obj_bytes);
// }