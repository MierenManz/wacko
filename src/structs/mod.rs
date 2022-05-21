mod external_kind;
mod global;
mod instructions;
mod memory;
mod value_type;

pub use external_kind::ExternalKind;
pub use global::GlobalDescriptor;
pub use instructions::Instruction;
pub use memory::ResizableLimits;
pub use value_type::ValType;
