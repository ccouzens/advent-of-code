(module
  (memory (export "mem") 1)

  (func (export "floorCalculator") (param $length i32) (result i32)
    (local $floor i32)
    (local $index i32)

    (loop $loop
      (if (i32.lt_u (local.get $index) (local.get $length))
        (then
          (local.set $floor (i32.add (local.get $floor)
            (select
              (i32.const 1)
              (i32.const -1)
              (i32.eq (i32.load8_s (local.get $index)) (i32.const 40))
            )
          ))
          (local.set $index (i32.add (local.get $index) (i32.const 1)))
          (br $loop)
        )
      )
    )

    (return (local.get $floor))
  )
)
