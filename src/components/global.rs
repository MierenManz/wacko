use super::ValType;

#[derive(Debug)]
pub struct Global {
    pub(crate) is_mut: bool,
    pub(crate) val_type: ValType,
    /// Depending on `val_type` 1 to 16 bytes will be used
    pub(crate) value: [u8; 16],
}

impl Global {
    pub fn new_i32(val: i32, is_mut: bool) -> Global {
        let mut value: [u8; 16] = [0; 16];
        value.copy_from_slice(&val.to_le_bytes());

        Self {
            is_mut,
            val_type: ValType::I32,
            value,
        }
    }

    pub fn new_i64(val: i64, is_mut: bool) -> Self {
        let mut value: [u8; 16] = [0; 16];
        value.copy_from_slice(&val.to_le_bytes());

        Self {
            is_mut,
            val_type: ValType::I64,
            value,
        }
    }

    pub fn new_f32(val: f32, is_mut: bool) -> Global {
        let mut value: [u8; 16] = [0; 16];
        value.copy_from_slice(&val.to_le_bytes());

        Self {
            is_mut,
            val_type: ValType::F32,
            value,
        }
    }

    pub fn new_f64(val: f64, is_mut: bool) -> Self {
        let mut value: [u8; 16] = [0; 16];
        value.copy_from_slice(&val.to_le_bytes());

        Self {
            is_mut,
            val_type: ValType::F64,
            value,
        }
    }

    pub fn new_v128(value: [u8; 16], is_mut: bool) -> Self {
        Self { is_mut , val_type: ValType::V128, value }
    }
}
