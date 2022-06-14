use crate::Error;
use std::io::Write;

pub trait Section {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error>;
    fn id(&self) -> u8;
}
