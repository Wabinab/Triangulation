pub type Version = usize;

pub const REMINDER_TYPE: u64 = 0;
pub const UPDATE_VER: bool = true;

#[derive(Debug)]
pub enum CRUD {
  Create = 0,
  Update = 1,
  Delete = 2
}