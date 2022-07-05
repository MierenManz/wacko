(module
    (memory 1 1)
    (func
        i32.const 0
        f32.load
        drop
        i32.const 0
        f32.load align=1
        drop
        i32.const 0
        f32.load align=2
        drop
        i32.const 0
        f32.load offset=2
        drop

        i32.const 0
        f32.const 0
        f32.store
        i32.const 0
        f32.const 0
        f32.store align=1
        i32.const 0
        f32.const 0
        f32.store align=2
        i32.const 0
        f32.const 0
        f32.store offset=2
    )

        (func
        i32.const 0
        f64.load
        drop
        i32.const 0
        f64.load align=1
        drop
        i32.const 0
        f64.load align=2
        drop
        i32.const 0
        f64.load align=4
        drop
        i32.const 0
        f64.load offset=2
        drop

        i32.const 0
        f64.const 0
        f64.store
        i32.const 0
        f64.const 0
        f64.store align=1
        i32.const 0
        f64.const 0
        f64.store align=2
        i32.const 0
        f64.const 0
        f64.store align=4
        i32.const 0
        f64.const 0
        f64.store offset=2
    )
)