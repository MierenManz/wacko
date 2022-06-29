(module
  (import "fn" "decl" (func (param i32 f32 i64 f64) (result i32 f32 i64 f64)))
  (import "fn" "nop" (func))
  (export "fn_decl" (func 0))
  (export "fn_nop" (func 1))
)