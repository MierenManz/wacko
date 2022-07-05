use crate::*;
#[test]
fn static_data() {
    let mut module = Module::new(true);

    let mut memory = Memory::new(1, None);
    let ptr1 = memory.push_slice("Hello World!".as_bytes());
    assert_eq!(ptr1, 0);
    let ptr2 = memory.push_slice("Hello Andreu :D".as_bytes());
    assert_eq!(ptr2, 12);

    module.add_memory(memory, None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("./testdata/memory/static_data.wasm").unwrap();
    assert_eq!(reference, output);
}
