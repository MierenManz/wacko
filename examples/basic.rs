use wacko::FnBody;
use wacko::Instruction;
use wacko::Module;
use wacko::ValType;

fn main() {
    let validate_module = true;
    let mut wasm_module = Module::new(validate_module);

    // Create a new function body
    let input_arguments = vec![ValType::I32, ValType::I32];
    let return_argument = vec![ValType::I32];
    let mut fn_body = FnBody::new(input_arguments, return_argument);

    let instructions = vec![
        Instruction::LocalGet(0),
        Instruction::LocalGet(1),
        Instruction::I32Add,
    ];

    fn_body.add_instructions(instructions);

    let export_name = Some("add");
    wasm_module.add_function(fn_body, export_name);

    let wasm_bytes = wasm_module.compile().unwrap();
    std::fs::write("out.wasm", wasm_bytes).unwrap();
}
