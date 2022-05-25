mod errors;
mod module;
mod section;
mod sections;
mod structs;
pub use errors::*;
pub use module::Module;
pub use section::Section;
pub use sections::*;
pub use structs::ExternalKind;
pub use structs::GlobalDescriptor;
pub use structs::Instruction;
pub use structs::ResizableLimits;
pub use structs::ValType;
pub use structs::FnBody;

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
