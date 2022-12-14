#[derive(Debug)]
pub struct Memory {
    pub(crate) min: u32,
    pub(crate) max: Option<u32>,
    pub(crate) data: Vec<u8>,
}

impl Memory {
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self {
            min,
            max,
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data.extend(data);
    }
}
