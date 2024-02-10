use crate::*;
use crate::obj_dto::Person;

pub(crate) fn echo(msg: Bytes) -> Result<Option<String>, String> {
    Ok(Some(String::from_utf8_lossy(&msg).to_string()))
}

pub(crate) fn _unused_eg_person(msg: Bytes) -> Result<Option<String>, String> {
    let _p: Person = serde_json::from_slice(&msg).unwrap();
    Ok(None)
}