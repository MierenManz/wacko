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
    /// `min` is the minimum amount of pages that are allocated at initialization
    ///
    /// Initial page count may be higher than `min` if the initialized data is more than the pagecount allows for
    ///
    /// `max` is maximum amount of pages available at runtime
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
        let minimum = if (self.min as usize) * 64 * 1024 > self.bytes.len() {
            self.min as u32
        } else {
            (self.bytes.len() as f64 / ((1024 * 64) as f64)).ceil() as u32
        };
        ResizableLimits {
            minimum,
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
