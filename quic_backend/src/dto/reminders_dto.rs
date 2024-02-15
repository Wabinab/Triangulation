/// We might not use full name for each field later. 
/// The exact mappings will be done later. For now, just use full. 
use crate::*;
use crate::json::helper::*;

// We'll move Reminder and Stages out later. 
// Reminder is only defined for each pipeline, not at stage level. 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Reminder {
  pub(crate) name: String,
  pub(crate) uuid: String,
  pub(crate) description: String,
  pub(crate) stages: Vec<Stage>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Stage {
  pub(crate) step: u64,
  pub(crate) name: String,
  // pub(crate) pipeline: Vec<Pipeline>
  pub(crate) pipeline: Value
}

pub(crate) fn val_to_stage(val: Value, locale: String) -> Stage {
  let name: String = get_by_locale(val["name"].clone(), locale);

  Stage {
    step: val["step"].as_u64().unwrap_or(0u64),
    name: name,
    pipeline: val["pipeline"].clone()
  }
}

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
  // Answers?       // a
}


// =====================================================
// The stuffs that was pass from frontend to backend. 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DemandPipeline {
  pub(crate) stage_step: u64,
  pub(crate) pipeline_id: u64,
  pub(crate) filename: String,
  // pub(crate) locale: String  // en, fr. Demands will be handled in frontend. Return none if cannot find. 
}