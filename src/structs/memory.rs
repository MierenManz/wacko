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
    pub(crate) fn encode(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        let flags = if self.maximum.is_some() { 0x01 } else { 0x00 };
        written += writer.write(&[flags])?;
        written += write::unsigned(writer, self.minimum as u64)?;

        if let Some(v) = self.maximum {
            written += write::unsigned(writer, v as u64)?;
        }
        Ok(written)
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
