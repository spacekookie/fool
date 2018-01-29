//! The workspace layout engine
//!
//! A layout is created for a screen resolution and re-created for each
//! screen resolution. It holds the items of a buffer but can decide to
//! rearrange them in their order if necessary.

use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, Mutex};
use state::buffer::Buffer;



/// A layout represents items in a list on screen
pub struct Layout {
    bf: Arc<Mutex<Buffer>>,
    res: (usize, usize),
    length: usize,
    view_start: usize,
    view_stop: usize,
}
impl Layout {
    pub fn new(bf: Arc<Mutex<Buffer>>, res: (usize, usize)) -> Layout {
        let len = bf.lock().unwrap().len();
        return Layout {
            bf,
            res,
            length: len,
            view_start: 0,
            view_stop: res.1,
        };
    }

    pub fn update(&mut self) {}
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut text = String::new();
        return write!(f, "{}", text);
    }
}
