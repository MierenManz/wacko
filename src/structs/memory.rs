use crate::Error;
use crate::ValidationError;
use leb128::write;
use std::io::Write;

#[derive(Copy, Clone, PartialEq)]
pub struct ResizableLimits {
    pub minimum: u32,
    pub maximum: Option<u32>,
}

impl ResizableLimits {
    pub(crate) fn encode(self, writer: &mut impl Write) -> Result<(), Error> {
        let flags = if self.maximum.is_some() { 0x01 } else { 0x00 };
        writer.write_all(&[flags])?;
        write::unsigned(writer, self.minimum as u64)?;

        if let Some(v) = self.maximum {
            write::unsigned(writer, v as u64)?;
        }
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if let Some(v) = self.maximum {
            if v < self.minimum {
                return Err(ValidationError::InvalidMemorySetting);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Memory {
    min: u16,
    max: Option<u16>,
    bytes: Vec<u8>,
}

impl Memory {
    /// `min` is minimum "heap" size in pages (this is the amount of pages that don't have static data in them)
    ///
    /// `max` is the maximum "heap" size in pages if the wasm memory is allowed to grow
    pub fn new(min: u16, max: Option<u16>) -> Self {
        Self {
            min,
            max,
            bytes: Vec::with_capacity((min as usize) * 64 * 1024),
        }
    }

    /// Push a byte of static data and return the wasm pointer of it
    pub fn push<T: Into<u8>>(&mut self, data: T) -> usize {
        self.bytes.push(data.into());
        self.bytes.len() - 1
    }
    /// Push a slice of static data and return the wasm pointer of it
    pub fn extend_from_slice(&mut self, slice: &[u8]) -> usize {
        self.bytes.extend(slice);
        self.bytes.len() - slice.len()
    }

    /// Push the content of a `IntoIterator<Item = u8>` and return the wasm pointer of it
    pub fn extend_from_iter<I: IntoIterator<Item = u8>>(&mut self, data: I) -> usize {
        let iter: Vec<u8> = data.into_iter().collect();
        self.extend_from_slice(&iter)
    }

    pub(crate) fn inner(&self) -> ResizableLimits {
        ResizableLimits {
            minimum: self.min as u32,
            maximum: self.max.map(|x| x as u32),
        }
    }

    pub(crate) fn mem_slice(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn encode_same_minmax() {
        let mut writer = Vec::new();
        ResizableLimits {
            minimum: 1,
            maximum: Some(1),
        }
        .encode(&mut writer)
        .unwrap();

        assert_eq!(writer, vec![0x01, 0x01, 0x01])
    }

    #[test]
    fn encode_no_max() {
        let mut writer = Vec::new();
        ResizableLimits {
            minimum: 1,
            maximum: None,
        }
        .encode(&mut writer)
        .unwrap();

        assert_eq!(writer, vec![0x00, 0x01])
    }

    #[test]
    fn validate_max_lt_min() {
        let res = ResizableLimits {
            minimum: 2,
            maximum: Some(1),
        }
        .validate()
        .unwrap_err();

        assert_eq!(res, ValidationError::InvalidMemorySetting)
    }
}
