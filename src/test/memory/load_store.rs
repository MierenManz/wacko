use crate::*;

#[test]
fn load_store_integers() {
    let mut module = Module::new(true);
    let mem = Memory::new(1, Some(1));
    module.add_memory(mem, None);

    let mut fn_body0 = FnBody::new(vec![], vec![]);
    fn_body0.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I32Load(4, 0),
        Instruction::I32Load(1, 0),
        Instruction::I32Load(2, 0),
        Instruction::I32Load(4, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store(4, 0),
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store(1, 0),
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store(2, 0),
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store(4, 2),
    ]);

    let mut fn_body1 = FnBody::new(vec![], vec![]);
    fn_body1.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I32Load8U(1, 0),
        Instruction::I32Load8U(1, 2),
        Instruction::I32Load8S(1, 0),
        Instruction::I32Load8S(1, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store8(1, 0),
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store8(1, 2),
    ]);

    let mut fn_body2 = FnBody::new(vec![], vec![]);
    fn_body2.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I32Load16U(2, 0),
        Instruction::I32Load16U(1, 0),
        Instruction::I32Load16U(2, 2),
        Instruction::I32Load16S(2, 0),
        Instruction::I32Load16S(1, 0),
        Instruction::I32Load16S(2, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store16(2, 0),
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store16(1, 0),
        Instruction::I32Const(0),
        Instruction::I32Const(0),
        Instruction::I32Store16(2, 2),
    ]);

    let mut fn_body3 = FnBody::new(vec![], vec![]);
    fn_body3.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I64Load(8, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load(4, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load(8, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store(8, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store(1, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store(2, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store(4, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store(8, 2),
    ]);

    let mut fn_body4 = FnBody::new(vec![], vec![]);
    fn_body4.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I64Load8U(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load8U(1, 2),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load8S(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load8S(1, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store8(1, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store8(1, 2),
    ]);

    let mut fn_body5 = FnBody::new(vec![], vec![]);
    fn_body5.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I64Load16U(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load16U(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load16U(2, 2),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load16S(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load16S(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load16S(2, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store16(2, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store16(1, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store16(2, 2),
    ]);

    let mut fn_body6 = FnBody::new(vec![], vec![]);
    fn_body6.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::I64Load32U(4, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32U(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32U(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32U(4, 2),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32S(4, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32S(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32S(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::I64Load32S(4, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store32(4, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store32(1, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store32(2, 0),
        Instruction::I32Const(0),
        Instruction::I64Const(0),
        Instruction::I64Store32(4, 2),
    ]);

    module.add_function(fn_body0, None);
    module.add_function(fn_body1, None);
    module.add_function(fn_body2, None);
    module.add_function(fn_body3, None);
    module.add_function(fn_body4, None);
    module.add_function(fn_body5, None);
    module.add_function(fn_body6, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("./testdata/memory/load_store_int.wasm").unwrap();
    assert_eq!(reference, output);
}

#[test]
fn load_store_floats() {
    let mut module = Module::new(true);
    let mem = Memory::new(1, Some(1));
    module.add_memory(mem, None);

    let mut fn_body0 = FnBody::new(vec![], vec![]);
    fn_body0.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::F32Load(4, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F32Load(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F32Load(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F32Load(4, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::F32Const(0f32),
        Instruction::F32Store(4, 0),
        Instruction::I32Const(0),
        Instruction::F32Const(0f32),
        Instruction::F32Store(1, 0),
        Instruction::I32Const(0),
        Instruction::F32Const(0f32),
        Instruction::F32Store(2, 0),
        Instruction::I32Const(0),
        Instruction::F32Const(0f32),
        Instruction::F32Store(4, 2),
    ]);

    let mut fn_body1 = FnBody::new(vec![], vec![]);
    fn_body1.add_instructions(vec![
        Instruction::I32Const(0),
        Instruction::F64Load(8, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F64Load(1, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F64Load(2, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F64Load(4, 0),
        Instruction::Drop,
        Instruction::I32Const(0),
        Instruction::F64Load(8, 2),
        Instruction::Drop,
        // Spacing
        Instruction::I32Const(0),
        Instruction::F64Const(0f64),
        Instruction::F64Store(8, 0),
        Instruction::I32Const(0),
        Instruction::F64Const(0f64),
        Instruction::F64Store(1, 0),
        Instruction::I32Const(0),
        Instruction::F64Const(0f64),
        Instruction::F64Store(2, 0),
        Instruction::I32Const(0),
        Instruction::F64Const(0f64),
        Instruction::F64Store(4, 0),
        Instruction::I32Const(0),
        Instruction::F64Const(0f64),
        Instruction::F64Store(8, 2),
    ]);

    module.add_function(fn_body0, None);
    module.add_function(fn_body1, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("./testdata/memory/load_store_float.wasm").unwrap();
    assert_eq!(reference, output);
}
