use crate::*;
#[test]
fn control_flow_test() {
    let mut module = Module::new(true);
    module.add_function(FnBody::new(vec![], vec![]), None);
    let mut fn_body = FnBody::new(vec![], vec![]);
    
    fn_body.add_local(ValType::I32);

    let instructions = vec![
      Instruction::Block(ValType::Void),
      Instruction::Loop(ValType::Void),
      Instruction::I32Const(10),
      Instruction::LocalGet(0),
      Instruction::I32Eq,
      Instruction::BrIf(0),
      Instruction::I32Const(1),
      Instruction::LocalGet(0),
      Instruction::I32Add,
      Instruction::LocalSet(0),
      Instruction::Br(0),
      Instruction::End,
      Instruction::End,
      Instruction::Call(0),
      Instruction::Nop,
      Instruction::I32Const(1),
      Instruction::If(ValType::Void),
      Instruction::Return,
      // Instruction::Else,
      Instruction::End,
      Instruction::I32Const(1),
      Instruction::If(ValType::I32),
      Instruction::I32Const(1),
      Instruction::Else,
      Instruction::I32Const(0),
      Instruction::End,
      Instruction::Drop,
      Instruction::I32Const(0),
      Instruction::If(ValType::Void),
      Instruction::End,
      Instruction::I32Const(2),
      Instruction::I32Const(0),
      Instruction::I32Const(1),
      Instruction::Select,
      Instruction::Drop,
    ];

    fn_body.add_instructions(instructions);

    module.add_function(fn_body, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/control_flow.wasm").unwrap();
    
    for x in 0..output.len() {
      println!("OUTPUT: {:?} ({:?}) \tRef: {:?} ({:?})\t{:?}", output[x], output[x] as *const u8, reference[x], reference[x] as *const u8, reference[x] == output[x])
    }
    assert_eq!(reference, output);
}
