;;! target = "x86_64"
;;! test = "winch"

(module
  (func (export "a") (param v128)
    local.get 0
    block
    end
    drop
  )
)
;; wasm[0]::function[0]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    8(%rdi), %r11
;;       movq    (%r11), %r11
;;       addq    $0x30, %r11
;;       cmpq    %rsp, %r11
;;       ja      0x4b
;;   1b: movq    %rdi, %r14
;;       subq    $0x20, %rsp
;;       movq    %rdi, 0x18(%rsp)
;;       movq    %rsi, 0x10(%rsp)
;;       movdqu  %xmm0, (%rsp)
;;       movdqu  (%rsp), %xmm15
;;       subq    $0x10, %rsp
;;       movdqu  %xmm15, (%rsp)
;;       addq    $0x10, %rsp
;;       addq    $0x20, %rsp
;;       popq    %rbp
;;       retq
;;   4b: ud2
