use std::any::Any;

use rustc_serialize::{Encodable, Encoder};

use types::progress::Progress;

pub trait Task: Any {
    fn progress(&self) -> Progress;
}

mopafy!(Task);

impl Encodable for Box<Task> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        if let Some(cb) = self.downcast_ref::<Checkbox>() {
            try!(encoder.emit_bool(cb.checked()));
            Ok(())
        } else {
            panic!("Unknown task type, cannot serialize")
        }
    }
}

#[derive(RustcEncodable)]
pub struct Checkbox(bool);

impl Checkbox {
    pub fn checked(&self) -> bool {
        self.0
    }
}

impl Task for Checkbox {
    fn progress(&self) -> Progress {
        Progress::new(if self.checked() { 100 } else { 0 }).unwrap()
    }
}
