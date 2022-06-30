use crate::ResizableLimits;

#[derive(Clone, Debug)]
pub struct Table {
    min: u32,
    max: Option<u32>,
    refs: Vec<u32>,
}

impl Table {
    /// `min` is the minimum amount of refs that are allocated at initialization
    ///
    /// Initial page count may be higher than `min` if the initialized data is more than the pagecount allows for
    ///
    /// `max` is maximum amount of refs available at runtime
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self {
            min,
            max,
            refs: Vec::with_capacity(min as usize),
        }
    }

    /// Push a function index to the table and return it's offset
    pub fn push(&mut self, data: u32) -> usize {
        self.refs.push(data);
        self.refs.len() - 1
    }

    pub(crate) fn refs(&self) -> &[u32] {
        &self.refs
    }

    pub(crate) fn inner(&self) -> ResizableLimits {
        let minimum = if self.min > self.refs.len() as u32 {
            self.min
        } else {
            self.refs.len() as u32
        };
        ResizableLimits {
            minimum,
            maximum: self.max,
        }
    }
}
