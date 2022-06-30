use crate::*;

#[test]
fn empty_memory_test() {
    let mut module = Module::new(true);
    let mem = Memory::new(0, Some(0));
    module.add_memory(mem, None);

    let output = module.compile().unwrap();
    let reference = std::fs::read("./testdata/memory/empty.wasm").unwrap();
    assert_eq!(reference, output);
}
