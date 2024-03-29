pub type Version = usize;
pub type Decimal = String;

pub const REMINDER_TYPE: u64 = 0;
pub const UPDATE_VER: bool = true;

pub const PROJECT_NAME: &'static str = "P";
pub const TEMPLATE_NAME: &'static str = "T";

#[derive(Debug)]
pub enum CRUD {
  Create = 0,
  Update = 1,
  Delete = 2
}