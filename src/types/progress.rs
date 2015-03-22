/// A number representing the completion percentage of a task, ranging from 0 to 100.
/// 100 indicates a completed task, but 0 does not necessarily mean that the task has not started.
/// Calculated progress values should generally be rounded down.
pub struct Progress(u8);

impl Progress {
    pub fn new(value: u8) -> Option<Progress> {
        if value > 100 { None } else { Some(Progress(value)) }
    }

    pub fn value(&self) -> &u8 {
        &self.0
    }
}
