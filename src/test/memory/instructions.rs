use crate::*;

#[test]
fn instructions_test() {
    let mut module = Module::new(true);
    let mem = Memory::new(1, Some(1));
    module.add_memory(mem, Some("mem"));
    let mut fn_body = FnBody::new(vec![], vec![]);

    fn_body.add_instructions(vec![
        Instruction::I32Const(1),
        Instruction::MemoryGrow,
        Instruction::MemorySize,
        Instruction::Drop,
        Instruction::Drop,
    ]);

    module.add_function(fn_body, None);

    let output = module.compile().unwrap();
    let reference = std::fs::read("./testdata/memory/instructions.wasm").unwrap();

    for i in output.clone() {
        println!("{:?}\t{:?}", i, i as *const u8);
    }
    assert_eq!(reference, output);
}
