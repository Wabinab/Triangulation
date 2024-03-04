/// We might not use full name for each field later. 
/// The exact mappings will be done later. For now, just use full. 
use crate::*;
use crate::json::helper::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitPipeline {
  pub(crate) stage_step: u64,
  pub(crate) pipeline_id: u64,
  pub(crate) filename: String,
  pub(crate) locale: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SubmitReminder {
  pub(crate) id: u64,
  pub(crate) stage_step: u64,
  pub(crate) filename: String,
  pub(crate) locale: String,
  pub(crate) t: u64,  // = 0 if reminder. 
  pub(crate) title: String,
  pub(crate) question: Value
}

pub(crate) trait SubmitReminderTrait {
  fn to_serde(&self, old_serde: Value) -> Option<Value>;
}

impl SubmitReminderTrait for SubmitReminder {
  fn to_serde(&self, old_serde: Value) -> Option<Value> {
      let mut new_serde = old_serde.clone();

      // Find step. 
      let g_opt = new_serde["stages"].as_array();
      if g_opt.is_none() { return None; }
      let g = g_opt.unwrap();
      let pos_opt = g.iter().position(|r| r["step"] == self.stage_step);
      if pos_opt.is_none() { return None; }
      let pos = pos_opt.unwrap();

      // No check, just insert. 
      let data = json!({
        "id": self.id,
        "t": self.t,
        "title": { self.locale.clone(): self.title.clone() },
        "question": self.question
      });
      let arr_len = new_serde["stages"]["pipeline"].as_array().unwrap().len();
      if (self.id - 1) < arr_len.try_into().unwrap() {
        let id: usize = (self.id - 1).try_into().unwrap();
        new_serde["stages"]["pipeline"][id] = data.clone();
      } else {
        let mut h = new_serde["stages"]["pipeline"].as_array().unwrap().clone();
        h.push(data);
        new_serde["stages"]["pipeline"] = json!(h);
      }
      
      Some(new_serde)
  }
}

// =============================================================
/// Pipeline part only. 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Pipeline {
  pub(crate) id: u16,
  pub(crate) t: u8,  // card type
  pub(crate) title: String,
  pub(crate) questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Question {
  pub(crate) q: String, // question
  pub(crate) t: u8,  // question type
  pub(crate) r: Option<Vec<String>>,
  pub(crate) c: Option<Vec<String>>,
  pub(crate) min: Option<u64>,
  pub(crate) max: Option<u64>,
  pub(crate) min_name: Option<String>,
  pub(crate) max_name: Option<String>
  // Answers?       // a
}
