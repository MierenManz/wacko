use crate::*;
#[test]
fn i64_test() {
    let mut module = Module::new(true);
    let mut fn_body = FnBody::new(vec![ValType::I32], vec![ValType::I32]);
    fn_body.add_local(ValType::I32);
    fn_body.add_local(ValType::I32);

    let instructions = vec![
      Instruction::LocalGet(0),
      Instruction::LocalTee(1),
      Instruction::LocalSet(2),
      Instruction::LocalGet(0),
      Instruction::End,
    ];

    fn_body.add_instructions(instructions);

    module.add_function(fn_body, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/local.wasm").unwrap();
    assert_eq!(reference, output);
}
