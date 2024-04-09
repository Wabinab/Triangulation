pub type Version = usize;
pub type Decimal = String;

pub const REMINDER_TYPE: u64 = 0;
pub const KELLY_TYPE: u64 = 1;
pub const CHECKLIST_TYPE: u64 = 2;

pub const UPDATE_VER: bool = true;

pub const PROJECT_NAME: &'static str = "P";
pub const TEMPLATE_NAME: &'static str = "T";

#[derive(Debug)]
pub enum CRUD {
  Create = 0,
  Update = 1,
  Delete = 2
}

#[derive(Debug)]
pub enum CloneType {
  Template = 0,
  Project = 1
}