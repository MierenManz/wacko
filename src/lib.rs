mod errors;
mod module;
mod sections;
mod structs;
pub use errors::*;
pub use module::Module;
pub(crate) use sections::*;
pub use structs::ExternalKind;
pub use structs::FnBody;
pub use structs::GlobalDescriptor;
pub use structs::GlobalValue;
pub use structs::Instruction;
pub use structs::ResizableLimits;
pub use structs::ValType;

#[cfg(test)]
mod test {
    pub use crate::*;
    #[test]
    fn i32_test() {
        let mut module = Module::new(true);
        let mut fn_body = FnBody::new(vec![], vec![ValType::I32]);

        let instructions = vec![
            Instruction::I32Const(1),
            Instruction::I32Eqz,
            Instruction::I32Const(1),
            Instruction::I32Eq,
            Instruction::I32Const(1),
            Instruction::I32Ne,
            Instruction::I32Const(1),
            Instruction::I32ltS,
            Instruction::I32Const(1),
            Instruction::I32ltU,
            Instruction::I32Const(1),
            Instruction::I32GtS,
            Instruction::I32Const(1),
            Instruction::I32GtU,
            Instruction::I32Const(1),
            Instruction::I32leS,
            Instruction::I32Const(1),
            Instruction::I32leU,
            Instruction::I32Const(1),
            Instruction::I32GeS,
            Instruction::I32Const(1),
            Instruction::I32GeU,
            Instruction::I32Clz,
            Instruction::I32Ctz,
            Instruction::I32PopCnt,
            Instruction::I32Const(1),
            Instruction::I32Add,
            Instruction::I32Const(1),
            Instruction::I32Sub,
            Instruction::I32Const(1),
            Instruction::I32Mul,
            Instruction::I32Const(1),
            Instruction::I32DivS,
            Instruction::I32Const(1),
            Instruction::I32DivU,
            Instruction::I32Const(1),
            Instruction::I32RemS,
            Instruction::I32Const(1),
            Instruction::I32RemU,
            Instruction::I32Const(1),
            Instruction::I32And,
            Instruction::I32Const(1),
            Instruction::I32Or,
            Instruction::I32Const(1),
            Instruction::I32Xor,
            Instruction::I32Const(1),
            Instruction::I32Shl,
            Instruction::I32Const(1),
            Instruction::I32ShrS,
            Instruction::I32Const(1),
            Instruction::I32ShrU,
            Instruction::I32Const(1),
            Instruction::I32Rotl,
            Instruction::I32Const(1),
            Instruction::I32Rotr,
            Instruction::I64Const(1),
            Instruction::I32WrapI64,
            Instruction::Drop,
            Instruction::F32Const(1f32),
            Instruction::I32TruncF32S,
            Instruction::Drop,
            Instruction::F32Const(1f32),
            Instruction::I32TruncF32U,
            Instruction::Drop,
            Instruction::F64Const(1f64),
            Instruction::I32TruncF64S,
            Instruction::Drop,
            Instruction::F64Const(1f64),
            Instruction::I32TruncF64U,
            Instruction::Drop,
            Instruction::F32Const(1f32),
            Instruction::I32ReinterpretF32,
            Instruction::Drop,
            Instruction::I32Extend8S,
            Instruction::I32Extend16S,
            Instruction::Drop,
            Instruction::F32Const(1f32),
            Instruction::I32TruncSatF32S,
            Instruction::Drop,
            Instruction::F32Const(1f32),
            Instruction::I32TruncSatF32U,
            Instruction::Drop,
            Instruction::F64Const(1f64),
            Instruction::I32TruncSatF64S,
            Instruction::Drop,
            Instruction::F64Const(1f64),
            Instruction::I32TruncSatF64U,
            Instruction::End,
        ];

        fn_body.add_instructions(instructions);

        module.add_function(fn_body, None).unwrap();
        let output = module.compile().unwrap();
        let reference = std::fs::read("testdata/fn/i32.wasm").unwrap();
        assert_eq!(reference, output);
    }

}
