use std::io::Write;
use crate::ExternalKind;
use crate::ValType;
use crate::Section;

pub struct ExportSection {
    fn_idx_space: Vec<(String, ExternalKind)>,
    table_idx_space: Vec<(String, ExternalKind)>,
    mem_idx_space: Vec<(String, ExternalKind)>,
    global_idx_space: Vec<(String, ExternalKind)>
}