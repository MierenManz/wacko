use crate::*;

#[test]
fn import_export_test() {
    let mut module = Module::new(true);
    let type_idx1 = module.add_type(vec![ValType::I32, ValType::I64, ValType::F32, ValType::F64], vec![ValType::I32, ValType::I64, ValType::F32, ValType::F64]);
    let fn_index1 = module.add_fn_decl(type_idx1);
    let idx1 = module.add_import("fn", "decl", ExternalKind::Function(fn_index1));
    module.add_export(ExportKind::Function(idx1), "fn_decl");

    let type_idx2 = module.add_type(vec![], vec![]);
    let fn_index2 = module.add_fn_decl(type_idx2);
    let idx2 = module.add_import("fn", "nop", ExternalKind::Function(fn_index2));
    module.add_export(ExportKind::Function(idx2), "fn_nop");
    module.compile().unwrap();
}