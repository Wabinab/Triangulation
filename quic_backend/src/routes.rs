use crate::*;

pub(crate) fn routes_handler(msg: Bytes, path: String, data_path: PathBuf) -> Result<Option<String>, String> {
    return match path.as_str() {
        // "/" => home_controller::echo(msg),

        // "/template" => template_controller::get_template(data_path, msg),
        "/template/nlist" => template_controller::get_template_nlist(data_path, msg),
        "/templates" => template_controller::get_templates_nameonly(data_path),
        "/template/new" => template_controller::new_template(data_path, msg),
        "/template/edit" => template_controller::edit_template(data_path, msg),
        "/template/version/newest" => template_controller::get_template_version(data_path, msg),

        // "/template/pipeline/reminder/save" => template_controller::save_reminder(data_path, msg),
        "/pipeline" => pipeline_controller::get_pipeline(data_path, msg),
        "/pipeline/0/new" => pipeline_controller::new_pipeline(data_path, msg, 0),
        "/pipeline/0/edit" => pipeline_controller::edit_pipeline(data_path, msg, 0),
        "/pipeline/0/delete" => pipeline_controller::delete_pipeline(data_path, msg, 0),

        "/project" => project_controller::get_project(data_path, msg),
        "/project/new" => project_controller::new_project(data_path, msg),
        "/project/edit" => project_controller::edit_project(data_path, msg),
        _ => Err("Cannot find route.".to_owned())
    };
}