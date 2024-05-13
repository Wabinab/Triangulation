use crate::*;

use self::messages::NO_ROUTE;


pub(crate) fn routes_u8(msg: Bytes, path: String, data_path: PathBuf) -> Result<Option<Vec<u8>>, String> {
  return match routes_handler(msg.clone(), path.clone(), data_path.clone()) {
    Ok(Some(value)) => Ok(Some(value.into_bytes())),
    Ok(None) => Ok(None),
    Err(err) => {
      if err == NO_ROUTE.to_owned() { return routes_handler_byte(msg, path, data_path); }
      else { return Err(err); }
    },
  }
}

pub(crate) fn routes_handler(msg: Bytes, path: String, data_path: PathBuf) -> Result<Option<String>, String> {
    return match path.as_str() {
        // "/" => home_controller::echo(msg),

        // "/template" => template_controller::get_template(data_path, msg),
        "/template/nlist" => template_controller::get_template_nlist(data_path, msg),
        "/template/new" => template_controller::new_template(data_path, msg),
        "/template/edit" => template_controller::edit_template(data_path, msg),
        "/template/delete" => template_controller::delete_template(data_path, msg),
        "/template/clone" => template_controller::clone_template(data_path, msg),
        "/template/version/newest" => template_controller::get_template_version(data_path, msg),
        "/templates" => template_controller::get_templates(data_path, msg),
        "/templates/nameonly" => template_controller::get_templates_nameonly(data_path),

        // "/template/pipeline/reminder/save" => template_controller::save_reminder(data_path, msg),
        "/pipeline" => pipeline_controller::get_pipeline(data_path, msg),
        "/pipeline/proj" => pipeline_controller::get_pipeline_by_uuid_ver(data_path, msg),
        "/pipeline/0/new" => pipeline_controller::new_pipeline(data_path, msg, CardTypes::Reminder),
        "/pipeline/0/edit" => pipeline_controller::edit_pipeline(data_path, msg, CardTypes::Reminder),
        "/pipeline/0/delete" => pipeline_controller::delete_pipeline(data_path, msg, CardTypes::Reminder),
        "/pipeline/1/new" => pipeline_controller::new_pipeline(data_path, msg, CardTypes::Kelly),
        "/pipeline/1/edit" => pipeline_controller::edit_pipeline(data_path, msg, CardTypes::Kelly),
        // NOTE: using /pipeline/0/delete is the same for all. Can deprecate it if possible later. 
        // "/pipeline/1/delete" => pipeline_controller::delete_pipeline(data_path, msg, CardTypes::Kelly),
        "/pipeline/2/new" => pipeline_controller::new_pipeline(data_path, msg, CardTypes::Checklist),
        "/pipeline/2/edit" => pipeline_controller::edit_pipeline(data_path, msg, CardTypes::Checklist),

        "/project" => project_controller::get_project(data_path, msg),
        "/project/new" => project_controller::new_project(data_path, msg),
        "/project/edit" => project_controller::edit_project(data_path, msg),
        "/project/delete" => project_controller::delete_project(data_path, msg),
        "/project/clone" => project_controller::clone_project(data_path, msg),
        "/project/edit/unsafe_ver" => project_controller::edit_version_unsafe(data_path, msg),
        "/projects" => project_controller::get_projects(data_path, msg),

        "/response" => response_controller::get_response(data_path, msg, CardTypes::Reminder),
        "/response/checklist" => response_controller::get_response(data_path, msg, CardTypes::Checklist),
        "/response/edit" => response_controller::edit_response(data_path, msg, CardTypes::Reminder),
        "/response/delete" => response_controller::delete_response(data_path, msg, CardTypes::Reminder),
        "/response/edit/kelly" => response_controller::edit_response(data_path, msg, CardTypes::Kelly),
        "/response/delete/kelly" => response_controller::delete_response(data_path, msg, CardTypes::Kelly),
        "/response/edit/checklist" => response_controller::edit_response(data_path, msg, CardTypes::Checklist),
        "/response/delete/checklist" => response_controller::delete_response(data_path, msg, CardTypes::Checklist),

        "/cycle/new" => cycle_controller::modify_cycle(data_path, msg, CRUD::Create),
        "/cycle/edit" => cycle_controller::modify_cycle(data_path, msg, CRUD::Update),
        "/cycle/delete" => cycle_controller::modify_cycle(data_path, msg, CRUD::Delete),
        "/cycle/clear" => cycle_controller::modify_cycle(data_path, msg, CRUD::Clear),

        // If you use "/sample_templ", it won't work for unknown reason. 
        "/sample/list" => sample_controller::get_downloaded_list(data_path),
        "/sample/nlist" => sample_controller::get_sample_nlist(data_path, msg),
        "/sample/pipeline" => sample_controller::get_sample_pipeline(data_path, msg),
        "/sample/clone" => sample_controller::clone_sample_template(data_path, msg),
        "/sample/download" => sample_controller::download_sample_template(data_path, msg),

        // Miscellaneous functions
        "/gen_filename" => misc_controller::get_filename(msg),

        _ => Err(NO_ROUTE.to_owned())
    };
}

// This is to return bytes direction, without conversion to string. 
pub(crate) fn routes_handler_byte(msg: Bytes, path: String, data_path: PathBuf) -> Result<Option<Vec<u8>>, String> {
  return match path.as_str() {
    "/template/export" => template_controller::export_template(data_path, msg),
    _ => Err(NO_ROUTE.to_owned())
  }
}