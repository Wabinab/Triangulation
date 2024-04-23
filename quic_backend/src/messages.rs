pub(crate) const CANNOT_FIND_VER: &'static str = "backend.GetVer";
pub(crate) const TEMPLATE_CANNOT_NULL: &'static str = "backend.TemplateNull";
pub(crate) const OOB_STAGE_IDX: &'static str = "backend.OOBStage";
pub(crate) const OOB_PIPELINE_IDX: &'static str = "backend.OOBPipeline";
// pub(crate) const OOB_REMINDER_IDX: &'static str = "backend.OOBReminder";
pub(crate) const REMINDER_IDX_CANNOT_NULL: &'static str = "backend.ReminderIdxNull";
pub(crate) const PIPELINE_IDX_CANNOT_NULL: &'static str = "backend.PipelineIdxNull";
pub(crate) const LEN_PIPELINE_NOT_MATCH: &'static str = "backend.PipelineLen";
pub(crate) const ANS_NONE: &'static str = "backend.AnswerNull";
pub(crate) const OOB_MIGRATE: &'static str = "backend.OOBMigrate";
pub(crate) const VER_TEMP_NONE: &'static str = "backed.VerTempNull";
pub(crate) const UUID_NO_NULL: &'static str = "backend.UuidNoNull";
pub(crate) const FILENAME_NO_NULL: &'static str = "backend.FilenameNoNull";
pub(crate) const COMPRESS_PACKING: &'static str = "backend.CompressPacking";
pub(crate) const COMPRESS_FLATE2: &'static str = "backend.CompressFlate2";
pub(crate) const COMPRESS_CFILE: &'static str = "backend.CompressCFile";
pub(crate) const COMPRESS_WFILE: &'static str = "backend.CompressWFile";
pub(crate) const RD_CANNOT_FIND_FILE: &'static str = "backend.RDNoFile";
pub(crate) const RD_DECOMPRESS: &'static str = "backend.RDDecompress";
pub(crate) const RD_UNPACKING: &'static str = "backend.RDUnpacking";
pub(crate) const RD_CONVJSON: &'static str = "backend.RDConvJson";
pub(crate) const UPD_VER_PROJ_FILE: &'static str = "backend.UVPFile";
pub(crate) const TITLE_NONE: &'static str = "backend.TitleNone";
pub(crate) const QUESTION_NONE: &'static str = "backend.QuestionNone";
pub(crate) const TRANSACTION_NONE: &'static str = "backend.TransactionNone";

pub(crate) const CHECKLIST_NONE: &'static str = "Checklist cannot be null";
pub(crate) const CYCLE_IDX_CANNOT_NULL: &'static str = "Cycle Index cannot be null";
pub(crate) const OOB_CYCLE_IDX: &'static str = "OOB Cycle Index";
pub(crate) const CYCLE_NAME_NULL: &'static str = "Cycle Name cannot be null nor empty string.";
pub(crate) const NOT_IMPLEMENTED: &'static str = "Function Not Implemented.";
pub(crate) const CYCLE_AT_LEAST_ONE: &'static str = "Must have at least 1 cycle.";
pub(crate) const VEC_BOOL_ONLY: &'static str = "Must be array of true/false only.";
pub(crate) const CHECKLIST_LEN_2: &'static str = "Bug: Checklist Extra length not = 2.";
pub(crate) const CHECKLIST_STRVEC: &'static str = "Bug: Checklist Extra question array not pure string.";
pub(crate) const CL_EXTRA_LEN_NOT_MATCH: &'static str = "Checklist Extra question and answer array length not match.";