bss {
  message: .asciz "Hello, world!"
  .asciz "Not sure why you'd do this..."
  error: .asciz "I'm sorry, Dave, but I can't open the pod bay doors."
}

raw {
  factorial:
    ENT 0x0
    REL 0x0
    LI
    PSH
    IMM 0x0
    EQ
    BZ 0x1d
    IMM 0x1
    RET
    REL 0x0
    LI
    PSH
    IMM 0x1
    SUB
    PUSHARG
    JSR factorial
    ADJ 0x1
    PSH
    REL 0x0
    LI
    MUL
    RET
    RET

  main:
    ENT 0x0
    IMM message
    PUSHARG
    INT 0x2      ; printstring()
    ADJ 0x1
    IMM 0x7
    PUSHARG
    JSR factorial
    ADJ 0x1
    PSH
    INT 0x1      ; printint()
    IMM 0x0
    RET
    RET
}
