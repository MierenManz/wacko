use crate::*;
#[test]
fn empty_fnbody_test() {
    let mut module = Module::new(true);
    module.add_function(FnBody::new(vec![], vec![]), None);
    let output = module.compile().unwrap();
    let reference = std::fs::read("testdata/functions/empty_fn.wasm").unwrap();
    
    for x in 0..output.len() {
      println!("OUTPUT: {:?} ({:?}) \tRef: {:?} ({:?})\t{:?}", output[x], output[x] as *const u8, reference[x], reference[x] as *const u8, reference[x] == output[x])
    }
    assert_eq!(reference, output);
}
