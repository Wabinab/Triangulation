use crate::*;


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Person {
    name: String,
    age : u8,
    // phones: Vec<String>
    tablename: Option<String>
}