(module
    (memory 1 1)
    (func
        i32.const 1
        memory.grow
        memory.size
        drop
        drop   
    )

    (export "mem" (memory 0))
)