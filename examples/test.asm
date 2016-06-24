bss {
  message: .asciz "Hello, world!"
  .asciz "Not sure why you'd do this..."
  error: .asciz "I'm sorry, Dave, but I can't open the pod bay doors."
}

raw {
  main:
    ENT 0x0
    IMM message
    PUSHARG
    INT 0x2
    ADJ 0x1
    IMM 0x0
    RET
    RET
}
