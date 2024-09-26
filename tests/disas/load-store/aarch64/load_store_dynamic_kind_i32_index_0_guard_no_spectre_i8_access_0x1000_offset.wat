;;! target = "aarch64"
;;! test = "compile"
;;! flags = " -C cranelift-enable-heap-access-spectre-mitigation=false -O static-memory-maximum-size=0 -O static-memory-guard-size=0 -O dynamic-memory-guard-size=0"

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i32 1)

  (func (export "do_store") (param i32 i32)
    local.get 0
    local.get 1
    i32.store8 offset=0x1000)

  (func (export "do_load") (param i32) (result i32)
    local.get 0
    i32.load8_u offset=0x1000))

;; wasm[0]::function[0]:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       ldr     x11, [x2, #0x68]
;;       mov     w12, w4
;;       mov     x13, #0x1001
;;       sub     x11, x11, x13
;;       cmp     x12, x11
;;       cset    x12, hi
;;       uxtb    w12, w12
;;       cbnz    x12, #0x3c
;;   28: ldr     x13, [x2, #0x60]
;;       add     x13, x13, #1, lsl #12
;;       strb    w5, [x13, w4, uxtw]
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;   3c: .byte   0x1f, 0xc1, 0x00, 0x00
;;
;; wasm[0]::function[1]:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       ldr     x11, [x2, #0x68]
;;       mov     w12, w4
;;       mov     x13, #0x1001
;;       sub     x11, x11, x13
;;       cmp     x12, x11
;;       cset    x12, hi
;;       uxtb    w12, w12
;;       cbnz    x12, #0x7c
;;   68: ldr     x13, [x2, #0x60]
;;       add     x12, x13, #1, lsl #12
;;       ldrb    w2, [x12, w4, uxtw]
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;   7c: .byte   0x1f, 0xc1, 0x00, 0x00
