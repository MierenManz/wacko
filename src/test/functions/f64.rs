use crate::*;
#[test]
fn f64_test() {
    let mut module = Module::new(true);
    let mut fn_body = FnBody::new(vec![], vec![ValType::F64]);

    let instructions = vec![
        Instruction::F64Const(1f64),
        Instruction::F64Const(1f64),
        Instruction::F64Eq,
        Instruction::Drop,
        Instruction::F64Const(1f64),
        Instruction::F64Const(1f64),
        Instruction::F64Ne,
        Instruction::Drop,
        Instruction::F64Const(1f64),
        Instruction::F64Const(1f64),
        Instruction::F64Lt,
        Instruction::Drop,
        Instruction::F64Const(1f64),
        Instruction::F64Const(1f64),
        Instruction::F64Gt,
        Instruction::Drop,
        Instruction::F64Const(1f64),
        Instruction::F64Const(1f64),
        Instruction::F64Le,
        Instruction::Drop,
        Instruction::F64Const(1f64),
        Instruction::F64Const(1f64),
        Instruction::F64Ge,
        Instruction::Drop,
        Instruction::F64Const(1f64),
        Instruction::F64Abs,
        Instruction::F64Neg,
        Instruction::F64Ceil,
        Instruction::F64Floor,
        Instruction::F64Trunc,
        Instruction::F64Const(1f64),
        Instruction::F64Add,
        Instruction::F64Const(1f64),
        Instruction::F64Sub,
        Instruction::F64Const(1f64),
        Instruction::F64Mul,
        Instruction::F64Const(1f64),
        Instruction::F64Div,
        Instruction::F64Const(1f64),
        Instruction::F64Min,
        Instruction::F64Const(1f64),
        Instruction::F64Max,
        Instruction::F64Const(1f64),
        Instruction::F64Copysign,
        Instruction::Drop,
        Instruction::I32Const(1),
        Instruction::F64ConvertI32S,
        Instruction::Drop,
        Instruction::I32Const(1),
        Instruction::F64ConvertI32U,
        Instruction::Drop,
        Instruction::I64Const(1),
        Instruction::F64ConvertI64S,
        Instruction::Drop,
        Instruction::I64Const(1),
        Instruction::F64ConvertI64U,
        Instruction::Drop,
        Instruction::F32Const(1f32),
        Instruction::F64PromoteF32,
        Instruction::Drop,
        Instruction::I64Const(1),
        Instruction::F64ReinterpretI64,
    ];

    fn_body.add_instructions(instructions);

    module.add_function(fn_body, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/f64.wasm").unwrap();

    for x in 0..output.len() {
        println!(
            "{:?} ({:?}) \t{:?} ({:?}) \t{:?}",
            reference[x],
            reference[x] as *const u8,
            output[x],
            output[x] as *const u8,
            reference[x] == output[x]
        );
    }
    println!("{:?} {:?}", reference.len(), output.len());
    assert_eq!(reference, output);
}
