use crate::Error;
use crate::Instruction;
use crate::Section;
use leb128::write;
use std::collections::HashMap;
use std::io::Write;

pub struct ElementSection {
    /// `HashMap<table_index, (offset, Vec<function_id>)>`
    table_elements: HashMap<u32, (i32, Vec<u32>)>,
}

impl ElementSection {
    pub fn new() -> Self {
        Self {
            table_elements: HashMap::new(),
        }
    }

    pub fn add_elements(&mut self, table: u32, element_offset: i32, elements: Vec<u32>) {
        if self.table_elements.contains_key(&table) {
            let (offset, mut elems) = self.table_elements.remove(&table).unwrap();
            elems.extend(elements);
            self.table_elements.insert(table, (offset, elems));
        } else {
            self.table_elements
                .insert(table, (element_offset, elements));
        }
    }
}

impl Section for ElementSection {
    fn compile(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;
        written += writer.write(&[self.id()])?;
        written += write::unsigned(writer, self.table_elements.len() as u64)?;
        for (table_idx, (offset, elements)) in self.table_elements {
            written += write::unsigned(writer, table_idx as u64)?;
            written += writer.write(&[(&Instruction::I32Const(offset)).into()])?;
            written += write::signed(writer, offset as i64)?;
            for idx in elements {
                written += write::unsigned(writer, idx as u64)?;
            }
        }

        Ok(written)
    }
    fn id(&self) -> u8 {
        0x09
    }
}

impl Default for ElementSection {
    fn default() -> Self {
        Self::new()
    }
}
