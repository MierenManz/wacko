#[derive(Copy, Clone)]
pub struct ResizableLimits {
    pub minimum: u32,
    pub maximum: Option<u32>,
}