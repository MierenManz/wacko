(module
  (func)
  (func
    (local i32)
    (block
      (loop
        i32.const 10
        local.get 0
        i32.eq
        br_if 0

        i32.const 1
        local.get 0
        i32.add
        local.set 0

        br 0
      )
    )

    call 0
    nop

    i32.const 1
    (if 
      (then
        return
      )
      (else)
    )

    i32.const 1
    (if (result i32)
      (then
        i32.const 1
      )
      (else 
        i32.const 0
      )
    )
    drop
    i32.const 0
    (if
      (then)
    )
    i32.const 2
    i32.const 0
    i32.const 1
    select
    drop
  )
)