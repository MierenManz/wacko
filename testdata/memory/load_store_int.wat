(module
    (memory 1 1)
    ;; I32
    (func
        i32.const 0
        i32.load
        i32.load offset=0 align=1
        i32.load offset=0 align=2
        i32.load offset=2
        drop

        i32.const 0
        i32.const 0
        i32.store
        i32.const 0
        i32.const 0
        i32.store offset=0 align=1
        i32.const 0
        i32.const 0
        i32.store offset=0 align=2
        i32.const 0
        i32.const 0
        i32.store offset=2
    )
    ;; (Un)signed 8 (i32)
    (func
        i32.const 0
        i32.load8_u
        i32.load8_u offset=2
        i32.load8_s
        i32.load8_s offset=2
        drop

        i32.const 0
        i32.const 0
        i32.store8
        i32.const 0
        i32.const 0
        i32.store8 offset=2
    )
    ;; (Un)signed 16 (i32)
    (func
        i32.const 0
        i32.load16_u
        i32.load16_u align=1
        i32.load16_u offset=2
        i32.load16_s
        i32.load16_s align=1
        i32.load16_s offset=2
        drop

        i32.const 0
        i32.const 0
        i32.store16
        i32.const 0
        i32.const 0
        i32.store16 align=1
        i32.const 0
        i32.const 0
        i32.store16 offset=2
    )
    ;; I64
    (func
        i32.const 0
        i64.load
        drop
        i32.const 0
        i64.load offset=0 align=1
        drop
        i32.const 0
        i64.load offset=0 align=2
        drop
        i32.const 0
        i64.load offset=0 align=4
        drop
        i32.const 0
        i64.load offset=2
        drop

        i32.const 0
        i64.const 0
        i64.store
        i32.const 0
        i64.const 0
        i64.store offset=0 align=1
        i32.const 0
        i64.const 0
        i64.store offset=0 align=2
        i32.const 0
        i64.const 0
        i64.store offset=0 align=4
        i32.const 0
        i64.const 0
        i64.store offset=2
    )
    ;; ;; (Un)signed 8 (i64)
    (func
        i32.const 0
        i64.load8_u
        drop
        i32.const 0
        i64.load8_u offset=2
        drop
        i32.const 0
        i64.load8_s
        drop
        i32.const 0
        i64.load8_s offset=2
        drop

        i32.const 0
        i64.const 0
        i64.store8
        i32.const 0
        i64.const 0
        i64.store8 offset=2
    )
    ;; (Un)signed 16 (i64)
    (func
        i32.const 0
        i64.load16_u
        drop
        i32.const 0
        i64.load16_u align=1
        drop
        i32.const 0
        i64.load16_u offset=2
        drop
        i32.const 0
        i64.load16_s
        drop
        i32.const 0
        i64.load16_s align=1
        drop
        i32.const 0
        i64.load16_s offset=2
        drop

        i32.const 0
        i64.const 0
        i64.store16
        i32.const 0
        i64.const 0
        i64.store16 align=1
        i32.const 0
        i64.const 0
        i64.store16 offset=2
    )
    ;; ;; (Un)signed 32 (i64)
    (func
        i32.const 0
        i64.load32_u
        drop
        i32.const 0
        i64.load32_u align=1
        drop
        i32.const 0
        i64.load32_u align=2
        drop
        i32.const 0
        i64.load32_u offset=2
        drop
        i32.const 0
        i64.load32_s
        drop
        i32.const 0
        i64.load32_s align=1
        drop
        i32.const 0
        i64.load32_s align=2
        drop
        i32.const 0
        i64.load32_s offset=2
        drop

        i32.const 0
        i64.const 0
        i64.store32
        i32.const 0
        i64.const 0
        i64.store32 align=1
        i32.const 0
        i64.const 0
        i64.store32 align=2
        i32.const 0
        i64.const 0
        i64.store32 offset=2
    )
)