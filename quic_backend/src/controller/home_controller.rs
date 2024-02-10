use crate::*;

pub(crate) fn echo(msg: Bytes) -> Result<Option<String>, String> {
    Ok(Some(String::from_utf8_lossy(&msg).to_string()))
}

