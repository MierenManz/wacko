use crate::indices::FnIndex;

#[derive(Debug)]
pub struct Table {
    pub(crate) min: u32,
    pub(crate) max: Option<u32>,
    pub(crate) elements: Vec<FnIndex>,
}

impl Table {
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self {
            min,
            max,
            elements: Vec::new(),
        }
    }

    pub fn add_ref(&mut self, func_ref: FnIndex) {
        self.elements.push(func_ref);
    }
}
