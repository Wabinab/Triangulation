use crate::*;

pub(crate) fn routes_handler(msg: Bytes, path: String, data_path: PathBuf) -> Result<Option<String>, String> {
    return match path.as_str() {
        // "/" => home_controller::echo(msg),

        // "/template" => template_controller::get_template(data_path, msg),
        "/template/nlist" => template_controller::get_template_nlist(data_path, msg),
        "/template/new" => template_controller::new_template(data_path, msg),
        "/template/edit" => template_controller::edit_template(data_path, msg),
        "/template/version/newest" => template_controller::get_template_version(data_path, msg),
        "/templates" => template_controller::get_templates(data_path, msg),
        "/templates/nameonly" => template_controller::get_templates_nameonly(data_path),

        // "/template/pipeline/reminder/save" => template_controller::save_reminder(data_path, msg),
        "/pipeline" => pipeline_controller::get_pipeline(data_path, msg),
        "/pipeline/proj" => pipeline_controller::get_pipeline_by_uuid_ver(data_path, msg),
        "/pipeline/0/new" => pipeline_controller::new_pipeline(data_path, msg, 0),
        "/pipeline/0/edit" => pipeline_controller::edit_pipeline(data_path, msg, 0),
        "/pipeline/0/delete" => pipeline_controller::delete_pipeline(data_path, msg, 0),
        "/pipeline/1/new" => pipeline_controller::new_pipeline(data_path, msg, 1),
        "/pipeline/1/edit" => pipeline_controller::edit_pipeline(data_path, msg, 1),
        // NOTE: using /pipeline/0/delete is the same for all. Can deprecate it if possible later. 
        "/pipeline/1/delete" => pipeline_controller::delete_pipeline(data_path, msg, 1),

        "/project" => project_controller::get_project(data_path, msg),
        "/project/new" => project_controller::new_project(data_path, msg),
        "/project/edit" => project_controller::edit_project(data_path, msg),
        "/project/edit/unsafe_ver" => project_controller::edit_version_unsafe(data_path, msg),
        "/projects" => project_controller::get_projects(data_path, msg),

        "/response" => response_controller::get_response(data_path, msg),
        "/response/edit" => response_controller::edit_response(data_path, msg, None),
        "/response/delete" => response_controller::delete_response(data_path, msg, None),
        "/response/edit/kelly" => response_controller::edit_response(data_path, msg, Some("kelly".to_owned())),
        "/response/delete/kelly" => response_controller::delete_response(data_path, msg, Some("kelly".to_owned())),

        // Miscellaneous functions
        "/gen_filename" => misc_controller::get_filename(msg),
        _ => Err("Cannot find route.".to_owned())
    };
}