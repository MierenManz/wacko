# Definitions

## Table of Contents

- ### [Enums Definitons](#enums)
  - [ImportKind](#importkind)
  - [ExternalKind](#externalkind)
  - [ValType](#valtype)
  - [Instruction](#instruction)

- ### [Building Block Structs](#building-blocks)
  - [AnyIndex](#anyindex)
  - [Index](#index)
  - [Type](#type)
  - [Import](#import)
  - [Func](#function)
  - [Table](#table)
  - [Memory](#memory)
  - [Global](#global)
  - [Export](#export)

- ### [Module Struct](#module)
  - new
  - add_type
  - add_import
  - add_fn
  - add_table
  - set_memory
  - add_global
  - add_export
  - set_start
  - compile
  - compile_into

- ### [Validator Funcs](#validator-functions)
  - [validate](#validate)
  - [validate_imports](#validate-imports)
  - [validate_funcs](#validate-funcs)
  - [validate_tables](#validate-tables)
  - [validate_memory](#validate-memory)
  - [validate_globals](#validate-global)
  - [validate_exports](#validate-export)

- ### [Encoder Struct](#encoder)
  - new
  - encode
  - encode_into
  - encode_types
  - encode_imports
  - encode_funcs
  - encode_tables
  - encode_memory
  - encode_globals
  - encode_exports
  - encode_start
  - encode_elements
  - encode_code
  - encode_data
  - encode_instructions

## Enums

### ImportKind
The `ImportKind` enum is used for defining imported functions, tables, memory or globals.

Derives: `Debug` `Clone`\
Implements: Nothing

API:
```rs
pub enum ImportKind {
  /// Define a imported function
  Func(TypeIndex),
  /// Defining a imported table
  Table { min: u32, max: Option<u32> },
  /// Defining imported memory
  Memory { min: u32, max: Option<u32>, shared: bool },
  /// Defining a imported global
  Global { val_type: ValType, mutable: bool },
}
```

### ExternalKind
ExternalKind has 4 possible options used for importing and exporting.

Derive: `Copy` `Clone` `Debug`\
Implements: `From<ExternalKind> for u8`

```rs
pub(crate) ExternalKind {
  Func = 0x00,
  Table = 0x01,
  Memory = 0x02,
  Global = 0x03,
}
```

### ValType
All the possible value types. Used for validations

Derives: `Copy` `Clone` `Debug`\
Implements: `From<ExternalKind> for u8`

```rs
pub enum ValType {
  I32 = 0x7F,
  I64 = 0x7E,
  F32 = 0x7D,
  F64 = 0x7C,
  V128 = 0x7B,
  FuncRef = 0x70,
  ExternRef = 0x6F,
}
```

### Instruction
This describes all the possible instructions for webassembly

Derives: `Copy` `Clone` `Debug`\
Implements: Nothing

## Building Blocks
All these building blocks are made to easily work with parts of webassembly.
Everything is made to build up in a specific way.

### AnyIndex
This is a special struct only used internally

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub(crate) struct AnyIndex {
  kind: ExternalKind,
  rc: Rc<Cell<u32>>,
}
```
### Index
This is a group of wrappers where that all wrap over `Rc<Cell<u32>>` and some implement the `Into<AnyIndex>` trait

Derives: `Debug` `Clone`

- TypeIndex
- FnIndex     (implements `Into<AnyIndex>`)
- TableIndex  (implements `Into<AnyIndex>`)
- MemoryIndex (implements `Into<AnyIndex>`)
- GlobalIndex (implements `Into<AnyIndex>`)

Wacko 0.4.0 and lower had a footgun in it's design where it was possible to add imports after adding anything else.
This causes undefined behavior and possibly invalid binaries.
Therefore we changed the design so that these opaque values will always hold the correct internal value.
This internal value cannot be copied by users to avoid more undefined behavior.

### Type
The `Type` struct is used for constructing new types for functions and or branches.

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Type {
  pub(crate) params: Vec<ValType>,
  pub(crate) result: Vec<ValType>,
}

pub fn Type::new(params: Vec<ValType>, result: Vec<ValType>) -> Type;
```

### Import
The `Import` struct is used for defining imports in wasm

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Import {
  pub(crate) import_kind: ImportKind,
  pub(crate) namespace: String,
  pub(crate) external_name: String,
}

pub fn Import::new(namespace: String, external_name: String, import_kind: ImportKind) -> Import;
```

### Func
The `Func` struct is used for constructing new wasm functions.

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Func<'a> {
  pub(crate) instructions: Vec<Instruction<'a>>,
  pub(crate) locals: Vec<ValType>,
  pub(crate) type_idx: TypeIndex,
}

/// Create a new wasm function
pub fn Func::new(type_idx: &TypeIndex) -> Func;

pub fn Func::add_local(&mut self, kind: ValType) -> u32;

/// Add instructions to the wasm function
pub fn Func::add_instructions(&mut self, instr: &[Instruction<'a>]) -> ();
```

### Table
The `Table` struct is used for constructing jump table's in wasm.

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Table {
  pub(crate) min: u32,
  pub(crate) max: Option<u32>,
  pub(crate) elements: Vec<FnIndex>,
}

/// Create a new wasm table
pub fn Table::new(min: u32, max: Option<u32>) -> Table;

/// Add refs
pub fn Table::add_ref(&mut self, reference: FnIndex) -> ();
```

### Memory
The `Memory` struct is used for constructing addressable (linear) memory in wasm

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Memory {
  pub(crate) min: u32,
  pub(crate) max: Option<u32>,
  pub(crate) data: Vec<u8>,
}

/// Create a new wasm memory
pub fn Memory::new(min: u32, max: Option<u32>) -> Memory;

/// Add data to the memory
pub fn Memory::add_data(&mut self, data: &[u8]) -> ();
```

### Global
The `Global` struct is used for constructing globally accessible values in wasm

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Global {
  pub(crate) is_mut: bool,
  pub(crate) val_type: ValType,
  pub(crate) value: [u8; 16],
}

/// Create a new wasm global
pub fn Global::new_i32(value: i32, is_mut: bool) -> Global;
pub fn Global::new_i64(value: i64, is_mut: bool) -> Global;
pub fn Global::new_f32(value: f32, is_mut: bool) -> Global;
pub fn Global::new_f64(value: f64, is_mut: bool) -> Global;
pub fn Global::new_v128(value: [u8; 16], is_mut: bool) -> Global;
```

### Export
The `Export` struct is used to export any of the following types: `Func` `Table` `Memory` `Global`

Derives: `Debug`\
Implements: Nothing

API:
```rs
pub struct Export {
  pub(crate) export_name: String,
  pub(crate) index: AnyIndex, 
}

pub fn Export::func(idx: FnIndex, export_name: String) -> Export;
pub fn Export::table(idx: TableIndex, export_name: String) -> Export;
pub fn Export::memory(idx: MemoryIndex, export_name: String) -> Export;
pub fn Export::global(idx: GlobalIndex, export_name: String) -> Export;
```

## Module
The struct `Module` is the main struct used for turning ir into wasm bytes.

```rs
pub struct Module<'a> {
  pub(crate) types: Vec<Type>,
  pub(crate) imports: Vec<Import>,
  /// Includes code section
  pub(crate) funcs: Vec<Func<'a>>,
  /// Includes element section
  pub(crate) tables: Vec<Table>,
  /// Includes data section
  pub(crate) memory: Option<Memory>,
  pub(crate) globals: Vec<Global>,
  pub(crate) exports: Vec<Export>,
  /// Refers to a func in `Module.funcs`
  pub(crate) start_fn: Option<u32>,
}

pub fn Module::new(validate: bool, optimize: bool) -> Module;
pub fn Module::add_type(&mut self, fn_type: Type) -> TypeIndex;
pub fn Module::add_import(&mut self, import: Import) -> ();
pub fn Module::add_func(&mut self, func: Func<'a>) -> FnIndex;
pub fn Module::add_table(&mut self, table: Table) -> TableIndex;
pub fn Module::set_mem(&mut self, mem: Memory) -> MemoryIndex;
pub fn Module::add_global(&mut self, global: Global) -> GlobalIndex;
pub fn Module::add_export(&mut self, export: Export) -> ();
pub fn Module::set_start(&mut self, start_fn: FnIndex) -> ();
pub fn Module::compile(self) -> Result<Vec<u8>, Error>;
pub fn Module::compile_into::<W: std::io::Write>(self, writer: &mut W) -> Result<(), Error>;
```

## Validator Functions
This is a group of functions that are used to validate a `Module`
When done it wil return `Result<(), ValidationError>` which indicates if the module is valid wasm

### Validate
This is the only function exposed to the crate. The rest of them are private in the module.

API:
```rs
pub(crate) fn validate(module: &Module) -> Result<(), Vec<ValidationError>;
```

### Validate Imports
This is a private function used for validating all the imports inside of the `Module`

API:
```rs
fn validate_imports(imports: &[Import]) -> Vec<ValidationError>;
```


### Validate Funcs
This is a private function used for validating all the functions inside of the `Module`

API:
```rs
fn validate_funcs(funcs: &[Func]) -> Vec<ValidationError>;
```

### Validate Tables
This is a private function used for validating all the tables inside of the `Module`

API:
```rs
fn validate_tables(tables: &[Table]) -> Vec<ValidationError>;
```

### Validate Memory
This is a private function used for validating all the tables inside of the `Module`

API:
```rs
fn validate_memory(memory: &Memory) -> Vec<ValidationError>;
```

### Validate Global
This is a private function used for validating all the globals inside of the `Module`

API:
```rs
fn validate_globals(globals: &[Global]) -> Vec<ValidationError>;
```

### Validate Export
This is a private function used for validating all the exports inside of the `Module`

API:
```rs
fn validate_export(exports: &[Export]) -> Vec<ValidationError>;
```

## Encoder
This is a internal struct used for encoding a `Module` into a wasm binary. When this struct is

```rs
pub(crate) struct Encoder<W: std::io::Write> {
  module: Module,
}

pub(crate) fn Encoder::new(module: Module) -> Encoder;
pub(crate) fn Encoder::encode(self) -> Result<Vec<u8>, EncodingError>;
pub(crate) fn Encoder::encode_into(self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_types(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_imports(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_funcs(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_tables(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_memory(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_globals(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_exports(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_start(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_elements(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_code(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_data(&mut self, writer: &mut W) -> Result<(), EncodingError>;
fn Encoder::encode_instructions(&mut self, writer: &mut W) -> Result<(), EncodingError>;
```