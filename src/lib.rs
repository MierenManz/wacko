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
pub use structs::ResizableLimits;
pub use structs::ValType;
pub use structs::Instruction;

mod test {
    use super::ValType;
    use super::Instruction;
    #[test]
    fn test() {
        println!("Instruction size: {} bytes", std::mem::size_of::<Instruction>());
        println!("Value type size:  {} bytes", std::mem::size_of::<ValType>())
    }
}