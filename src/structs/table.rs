use crate::ResizableLimits;

#[derive(Clone, Debug)]
pub struct Table {
    min: u32,
    max: Option<u32>,
    refs: Vec<u32>,
}

impl Table {
    /// `min` is minimum "heap" size in pages (this is the amount of pages that don't have static data in them)
    ///
    /// `max` is the maximum "heap" size in pages if the wasm memory is allowed to grow
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self {
            min,
            max,
            refs: Vec::with_capacity(min as usize),
        }
    }

    /// Push a function index to the table and return it's offset
    pub fn push(&mut self, data: u32) -> usize {
        self.refs.push(data.into());
        self.refs.len() - 1
    }

    pub(crate) fn refs(&self) -> &[u32] {
        &self.refs
    }

    pub(crate) fn inner(&self) -> ResizableLimits {
        ResizableLimits {
            minimum: self.min,
            maximum: self.max,
        }
    }
}
