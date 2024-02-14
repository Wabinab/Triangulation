use crate::*;
use crate::obj_dto::Person;

use self::file::read_file;

pub(crate) fn echo(msg: Bytes) -> Result<Option<String>, String> {
    Ok(Some(String::from_utf8_lossy(&msg).to_string()))
}

pub(crate) fn _unused_eg_person(msg: Bytes) -> Result<Option<String>, String> {
    let _p: Person = serde_json::from_slice(&msg).unwrap();
    Ok(None)
}

// ===================================
pub(crate) fn get_sample_template(
    root: PathBuf, msg: Bytes
) -> Result<Option<String>, String> {
    let filename = String::from_utf8_lossy(&msg).to_string();
    let data = read_file(root.as_path(), filename);
    Ok(Some(String::from_utf8_lossy(&data).to_string()))
}