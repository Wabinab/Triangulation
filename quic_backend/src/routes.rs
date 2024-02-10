use crate::*;

pub(crate) fn routes_handler(input: Bytes, path: String) -> Result<Option<String>, String> {
    return match path.as_str() {
        "/" => home_controller::echo(input),
        _ => Ok(None)
    };
}