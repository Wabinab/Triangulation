use crate::*;

pub(crate) fn routes_handler(input: Bytes, path: String, data_path: PathBuf) -> Result<Option<String>, String> {
    return match path.as_str() {
        "/" => home_controller::echo(input),
        "/sample_template" => home_controller::get_sample_template(data_path, input),
        _ => Ok(None)
    };
}