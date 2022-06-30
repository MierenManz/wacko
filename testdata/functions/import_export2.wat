(module
  (import "fn" "decl" (func (param i32 f32 i64 f64) (result i32 f32 i64 f64)))
  (import "fn" "nop" (func))
  (func
    (param i32 i32)
    (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (export "fn_decl" (func 0))
  (export "fn_nop" (func 1))
  (export "add" (func 2))
)