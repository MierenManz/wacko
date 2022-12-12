use crate::Error;
use crate::Instruction;
use leb128::write;
use std::collections::HashMap;
use std::io::Write;

#[derive(Default)]
pub struct ElementSection {
    /// `HashMap<table_index, (offset, Vec<function_id>)>`
    table_elements: HashMap<u32, (i32, Vec<u32>)>,
}

impl ElementSection {
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

    pub fn compile(self, writer: &mut impl Write) -> Result<(), Error> {
        if self.table_elements.is_empty() {
            return Ok(());
        }
        writer.write_all(&[Self::id()])?;
        write::unsigned(writer, self.table_elements.len() as u64)?;
        for (table_idx, (offset, elements)) in self.table_elements {
            write::unsigned(writer, table_idx as u64)?;
            Instruction::I32Const(offset).write_opcode(writer)?;
            write::signed(writer, offset as i64)?;
            for idx in elements {
                write::unsigned(writer, idx as u64)?;
            }
        }

        Ok(())
    }
    fn id() -> u8 {
        0x09
    }
}
