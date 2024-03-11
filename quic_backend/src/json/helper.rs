// use crate::*;

// pub(crate) fn get_by_locale(val: Value, locale: String) -> String {
//   match val.as_object() {
//     Some(name) => {
//       if name.contains_key(&locale) { return name.get(&locale).unwrap().to_string(); }
//       let first_key = name.keys().next().unwrap();
//       return name.get(first_key).unwrap().to_string();
//     },
//     None => "".to_string()
//   }
// }

// /// val must be an array. 
// pub(crate) fn find_by_id(val: Value, id_name: &'static str, compare_val: u64) -> Option<Value> {
//   // sdata["stages"].as_array().unwrap()
//   //   .into_iter().find(|d| d["step"] == p.stage_step);
//   match val.as_array() {
//     Some(val2) => {
//       match val2.clone().into_iter().find(|d| d[id_name] == compare_val) {
//         Some(val3) => Some(val3),
//         None => None
//       }
//     },
//     None => None
//   }
// }