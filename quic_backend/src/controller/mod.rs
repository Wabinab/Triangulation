/// Inside the controller, anything that starts with `bi` means bidirectional_stream. 
/// And anything starts with `dg` means datagram. 

// https://stackoverflow.com/questions/69636018/how-to-import-a-function-from-another-folder-in-the-folder-above
pub(crate) mod template_controller;
pub(crate) mod pipeline_controller;
pub(crate) mod project_controller;
pub(crate) mod response_controller;
pub(crate) mod misc_controller;
pub(crate) mod cycle_controller;
pub(crate) mod sample_controller;