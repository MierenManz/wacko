use crate::*;
#[test]
fn f32_test() {
  let mut module = Module::new(true);
  let mut fn_body = FnBody::new(vec![], vec![ValType::F32]);

  let instructions = vec![
    Instruction::F32Const(1f32),
    Instruction::F32Const(1f32),
    Instruction::F32Eq,
    Instruction::Drop,
    Instruction::F32Const(1f32),
    Instruction::F32Const(1f32),
    Instruction::F32Ne,
    Instruction::Drop,
    Instruction::F32Const(1f32),
    Instruction::F32Const(1f32),
    Instruction::F32Lt,
    Instruction::Drop,
    Instruction::F32Const(1f32),
    Instruction::F32Const(1f32),
    Instruction::F32Gt,
    Instruction::Drop,
    Instruction::F32Const(1f32),
    Instruction::F32Const(1f32),
    Instruction::F32Le,
    Instruction::Drop,
    Instruction::F32Const(1f32),
    Instruction::F32Const(1f32),
    Instruction::F32Ge,
    Instruction::Drop,
    Instruction::F32Const(1f32),
    Instruction::F32Abs,
    Instruction::F32Neg,
    Instruction::F32Ceil,
    Instruction::F32Floor,
    Instruction::F32Trunc,
    Instruction::F32Const(1f32),
    Instruction::F32Add,
    Instruction::F32Const(1f32),
    Instruction::F32Sub,
    Instruction::F32Const(1f32),
    Instruction::F32Mul,
    Instruction::F32Const(1f32),
    Instruction::F32Div,
    Instruction::F32Const(1f32),
    Instruction::F32Min,
    Instruction::F32Const(1f32),
    Instruction::F32Max,
    Instruction::F32Const(1f32),
    Instruction::F32Copysign,
    Instruction::Drop,
    Instruction::I32Const(1),
    Instruction::F32ConvertI32S,
    Instruction::Drop,
    Instruction::I32Const(1),
    Instruction::F32ConvertI32U,
    Instruction::Drop,
    Instruction::I64Const(1),
    Instruction::F32ConvertI64S,
    Instruction::Drop,
    Instruction::I64Const(1),
    Instruction::F32ConvertI64U,
    Instruction::Drop,
    Instruction::F64Const(1f64),
    Instruction::F32DemoteF64,
    Instruction::Drop,
    Instruction::I32Const(1),
    Instruction::F32ReinterpretI32,
    Instruction::End,
  ];

  fn_body.add_instructions(instructions);

    module.add_function(fn_body, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/f32.wasm").unwrap();
    
    for x in 0..output.len() {
      println!("{:?} ({:?})\t{:?} ({:?})\t{:?}", reference[x], reference[x] as *const u8, output[x], output[x] as *const u8, reference[x] == output[x]);
    }
    println!("{:?} {:?}", reference.len(), output.len());
    assert_eq!(reference, output);
}