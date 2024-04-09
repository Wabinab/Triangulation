use uuid::Uuid;

use crate::{messages::{FILENAME_NO_NULL, UUID_NO_NULL}, *};

use self::{compressor::{compress_and_save, retrieve_decompress, retrieve_decompress_fullpath}, file::gen_filename, filelist_dto::SubmitFileList, project_dto::{to_basic_project, to_nlist_proj, ProjVerTrait, ProjectTrait, SubmitCloneProj, SubmitGetProject, SubmitProjVer, SubmitProject}, template_dto::to_nlist_temp, versioning::{get_savepath, get_verpath, upd_ver_proj}};

// ===========================================
// GET
/// Not only get the project data, but also its corresponding template nlist. 
/// This is already an "nlist", because we don't need a separate one. 
pub(crate) fn get_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitGetProject = serde_json::from_slice(&msg).unwrap();

  let proj = get_data(data_path.clone(), submit.filename.clone());
  if proj.is_err() { error!("get_project proj"); return Err(proj.unwrap_err()); }
  let proj = proj.unwrap();
  let proj_nlist = serde_json::to_value(
    &to_nlist_proj(proj.clone())
  );
  if proj_nlist.is_err() { error!("get_project proj_nlist"); return Err(proj_nlist.unwrap_err().to_string()); }

  // Get corresponding template. 
  let template_filename = gen_filename(
    TEMPLATE_NAME.to_owned(), 
    proj["t_uuid"].as_str().unwrap().to_owned(), 
    Some(proj["t_ver"].as_u64().unwrap() as usize)
  );
  let temp = retrieve_decompress(
    get_savepath(data_path), template_filename);
  if temp.is_err() { error!("get_project temp"); return Err(temp.unwrap_err()); }
  let temp_nlist = serde_json::to_value(
    &to_nlist_temp(temp.unwrap()));
  if temp_nlist.is_err() { error!("get_project temp_nlist"); return Err(temp_nlist.unwrap_err().to_string()); }

  Ok(Some(json!({
    // "project": proj_nlist.unwrap(),
    "project": proj_nlist.unwrap(),  // we need to do some testing. 
    "template": temp_nlist.unwrap()
  }).to_string()))
}

pub(crate) fn get_projects(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitFileList = serde_json::from_slice(&msg).unwrap();
  let paths: Vec<_> = fs::read_dir(modify_datapath(data_path.clone())).unwrap().collect();
  let mut errors: Vec<String> = vec![]; 
  let mut retvals: Vec<Value> = vec![];
  for path in paths
    .iter()
    .skip(submit.page_no * submit.page_size)
    .take(submit.page_size) 
  {
    let path = path.as_ref();
    if path.is_err() { error!("get_projects path as_ref"); errors.push(path.unwrap_err().to_string()); continue; }
    let data = retrieve_decompress_fullpath(path.unwrap().path());
    if data.is_err() { error!("get_projects retrieve proj"); errors.push(data.unwrap_err()); continue; }
    let mut project = to_basic_project(data.clone().unwrap());

    let proj = data.unwrap();
    let template_filename = gen_filename(
      TEMPLATE_NAME.to_owned(), 
      proj["t_uuid"].as_str().unwrap().to_owned(), 
      Some(proj["t_ver"].as_u64().unwrap() as usize)
    );
    let temp = retrieve_decompress(get_savepath(data_path.clone()), template_filename);
    if temp.is_err() { error!("get_projects retrieve temp"); return Err(temp.unwrap_err()); }
    let template = to_nlist_temp(temp.unwrap());
    project.t_name = Some(template.name);

    let retval = serde_json::to_value(&project);
    if retval.is_err() { error!("get_projects retval"); errors.push(retval.unwrap_err().to_string()); continue; }
    retvals.push(retval.unwrap());
  }

  Ok(Some(json!({
    "total_count": paths.len(),
    "data": retvals,
    "err": errors
  }).to_string()))
}

// ===========================================
// POST and PUT
pub(crate) fn new_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitProject = serde_json::from_slice(&msg).unwrap();
  if submit.template_uuid.is_none() { 
    error!("new_project template_uuid uuid cannot be null");
    return Err(UUID_NO_NULL.to_owned()); 
  }

  let uuid = Uuid::now_v7().to_string();
  let filename = gen_filename(PROJECT_NAME.to_string(), uuid.clone(), None);

  let template_filename = gen_filename(
    TEMPLATE_NAME.to_owned(), 
    submit.template_uuid.clone().unwrap(), 
    None
  );
  let version = upd_ver_proj(
    get_verpath(data_path.clone()), template_filename, data_path.clone());
  if version.is_err() { error!("new_project get_verpath"); return Err(version.unwrap_err()); }

  let template_filename = gen_filename(
    TEMPLATE_NAME.to_owned(), 
    submit.template_uuid.clone().unwrap(), 
    Some(version.clone().unwrap())
  );
  let temp = retrieve_decompress(
    get_savepath(data_path.clone()), template_filename);
  if temp.is_err() { error!("new_project retrieve_decompress"); return Err(temp.unwrap_err()); }

  let data = submit.new_project(uuid.clone(), version.unwrap(), temp.unwrap());
  if data.is_err() { error!("new_project new_project"); return Err(data.unwrap_err()); }

  let data = data.unwrap();
  let data_path = modify_datapath(data_path.clone());
  let ret = compress_and_save(
    data.to_string(), data_path, filename.clone());
  if ret.is_err() { error!("new_project compress_and_save"); return Err(ret.unwrap_err()); }

  Ok(Some(json!({ "filename": filename }).to_string()))
}


pub(crate) fn edit_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitProject = serde_json::from_slice(&msg).unwrap();
  if submit.filename.is_none() { error!("edit_project filename cannot be null."); 
    return Err(FILENAME_NO_NULL.to_owned()); }

  let old_serde = get_data(
    data_path.clone(), submit.filename.clone().unwrap());
  if old_serde.is_err() { error!("edit_project old_serde"); return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let mut new_templ_serde = None;
  if submit.version.is_some() {
    let temp_filename = gen_filename(TEMPLATE_NAME.to_owned(), 
      old_serde["t_uuid"].as_str().unwrap().to_string(), 
      Some(submit.version.unwrap().clone())
    );
    let temp = retrieve_decompress(
      get_savepath(data_path.clone()), temp_filename);
    if temp.is_err() { error!("edit_project temp inside version."); return Err(temp.unwrap_err()); }
    new_templ_serde = Some(temp.unwrap());
  }

  let edited_serde = submit.edit_project(old_serde, new_templ_serde);
  if edited_serde.is_err() { error!("edit project edited_serde"); return Err(edited_serde.unwrap_err()); } 
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
  modify_datapath(data_path.clone()), submit.filename.clone().unwrap());
  if ret.is_err() { error!("edit_project compress_and_save"); return Err(ret.unwrap_err()); }

  Ok(Some(edited_serde.unwrap().to_string()))
}

/// This is the unsafe saving of 'version' only. 
/// REPEAT: Save "version" ONLY!!! It's not saving the whole project!!!
/// Data loss at one's discretion. 
pub(crate) fn edit_version_unsafe(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitProjVer = serde_json::from_slice(&msg).unwrap();

  let old_serde = get_data(data_path.clone(), submit.filename.clone());
  if old_serde.is_err() { error!("edit_version_unsafe old_serde"); return Err(old_serde.unwrap_err()); }
  let old_serde = old_serde.unwrap();

  let temp_filename = gen_filename(TEMPLATE_NAME.to_owned(), 
    old_serde["t_uuid"].as_str().unwrap().to_string(), 
    Some(submit.version.clone())
  );
  let temp = retrieve_decompress(
    get_savepath(data_path.clone()), temp_filename);
  if temp.is_err() { error!("edit_version_unsafe temp"); return Err(temp.unwrap_err()); }

  let edited_serde = submit.edit_version(
    old_serde, temp.unwrap());
  if edited_serde.is_err() { error!("edit_version_unsafe edited_serde"); return Err(edited_serde.unwrap_err()); }
  let ret = compress_and_save(edited_serde.clone().unwrap().to_string(), 
    modify_datapath(data_path.clone()), submit.filename.clone());
  if ret.is_err() { error!("edit_version_unsafe compress_and_save"); return Err(ret.unwrap_err()); }

  Ok(Some(edited_serde.unwrap().to_string()))
}

pub(crate) fn delete_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitCloneProj = serde_json::from_slice(&msg).unwrap();
  let filename = gen_filename(PROJECT_NAME.to_owned(), submit.uuid, None);
  let mut filepath = modify_datapath(data_path);
  filepath.push(filename.clone());
  let ret = fs::remove_file(filepath.as_path());
  if ret.is_err() { error!("delete_project failed to remove file."); return Err(ret.unwrap_err().to_string()); }
  Ok(Some(json!({
    "msg": format!("Successfully delete project with filename: {}", filename)
  }).to_string()))
}

pub(crate) fn clone_project(data_path: PathBuf, msg: Bytes) -> Result<Option<String>, String> {
  let submit: SubmitCloneProj = serde_json::from_slice(&msg).unwrap();
  let filename = gen_filename(PROJECT_NAME.to_owned(), submit.uuid, None);
  let data = clone::clone_project(data_path, filename);
  if data.is_err() { error!("project_controller clone_project err."); return Err(data.unwrap_err()); }
  Ok(Some(data.unwrap().to_string()))
}

// ================================================
fn modify_datapath(data_path: PathBuf) -> PathBuf {
  file::modify_datapath(data_path, "project")
}

fn get_data(data_path: PathBuf, filename: String) -> Result<Value, String> {
  let data_path = modify_datapath(data_path);
  retrieve_decompress(data_path, filename)
}

// fn modify_datapath_temp(data_path: PathBuf) -> PathBuf {
//   let mut data_path = data_path;
//   data_path.push("template");
//   data_path
// }