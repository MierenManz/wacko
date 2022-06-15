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
    #[test]
    fn test() {
        use crate::Instruction;
        use crate::ValType;
        println!(
            "Instruction size: {} bytes",
            std::mem::size_of::<Instruction>()
        );
        println!("Value type size:  {} bytes", std::mem::size_of::<ValType>())
    }
}
