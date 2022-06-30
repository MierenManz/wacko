use crate::*;

#[test]
fn import_export_test() {
    let mut module = Module::new(true);
    let fnbody1 = FnBody::new(
        vec![ValType::I32, ValType::F32, ValType::I64, ValType::F64],
        vec![ValType::I32, ValType::F32, ValType::I64, ValType::F64],
    );
    let fn_index1 = module.import_function("fn", "decl", fnbody1);
    module.add_export(ExportKind::Function(fn_index1), "fn_decl");

    let fnbody2 = FnBody::new(vec![], vec![]);
    let fn_index2 = module.import_function("fn", "nop", fnbody2);
    module.add_export(ExportKind::Function(fn_index2), "fn_nop");
    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/import_export.wasm").unwrap();

    println!("{:?} {:?}", reference.len(), output.len());
    assert_eq!(reference, output);
}

#[test]
fn import_export2_test() {
    let mut module = Module::new(true);
    let fnbody1 = FnBody::new(
        vec![ValType::I32, ValType::F32, ValType::I64, ValType::F64],
        vec![ValType::I32, ValType::F32, ValType::I64, ValType::F64],
    );
    let fnbody2 = FnBody::new(vec![], vec![]);
    let mut fnbody3 = FnBody::new(vec![ValType::I32, ValType::I32], vec![ValType::I32]);

    fnbody3.add_instructions(vec![
        Instruction::LocalGet(0),
        Instruction::LocalGet(1),
        Instruction::I32Add,
    ]);

    let fn_index1 = module.import_function("fn", "decl", fnbody1);
    let fn_index2 = module.import_function("fn", "nop", fnbody2);

    module.add_export(ExportKind::Function(fn_index1), "fn_decl");
    module.add_export(ExportKind::Function(fn_index2), "fn_nop");
    module.add_function(fnbody3, Some("add"));

    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/import_export2.wasm").unwrap();
    for x in 0..reference.len() {
        print!("{:?} ({:?})", reference[x], reference[x] as *const u8,);
        if output.len() > x {
            print!(
                " \t{:?} ({:?}) \t{:?}\n",
                output[x],
                output[x] as *const u8,
                reference[x] == output[x]
            )
        } else {
            print!(" \t{:?}\n", false);
        }
    }
    println!("{:?} {:?}", reference.len(), output.len());
    assert_eq!(reference, output);
}
