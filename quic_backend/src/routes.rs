use crate::*;

pub(crate) fn routes_handler(msg: Bytes, path: String, data_path: PathBuf) -> Result<Option<String>, String> {
    return match path.as_str() {
        "/" => home_controller::echo(msg),
        "/sample_template" => home_controller::get_sample_template(data_path, msg),
        // Actually, it's an item in a pipeline, not the pipeline itself. We'll rename later. 
        "/pipeline" => home_controller::get_pipeline(data_path, msg),

        "/template/new" => template_controller::new_template(data_path, msg),
        "/template" => template_controller::get_template(data_path, msg),
        "/template/stages/edit" => template_controller::edit_stages(data_path, msg),
        _ => Ok(None)
    };
}