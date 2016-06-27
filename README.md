# sembler

[Stockfighter](https://www.stockfighter.io/) is a programming challenge involving automated trading and AVR microcontrollers. Sembler is an assembler for the virtual machine featured in Stockfighter. Sembler possibly contains spoilers for several Stockfighter levels, so read on at your own peril.

## Usage

```
sembler [OPTIONS] <INPUT>
```

Options are:

* `-e`, `--entry-point` `<SYMBOL>`: Allows you to specify a custom entry point. Defaults to `main`.
* `-o`, `--output` `<FILE>`: Allows you to specify an output file for the assembled bytecode. Defaults to printing output to standard output.

## Syntax

Assembly files are split into two sections, `bss` and `raw`. `bss` contains data, and `raw` contains executable instructions.

```
bss {
    ...
}

raw {
    ...
}
```

Each section consists of multiple statements. Each statement can either be an SVM (Stockfighter Virtual Machine) instruction, or a directive which tells the assembler to take some action.

Each statement can have a label preceding it. This label can then be used to refer to the address of the assembled statement in memory. For example:

```
adder:
    IMM 5
    ADD
```

`adder` refers to the address of the `IMM 5` instruction.

Some SVM instructions take an argument. These arguments can be either an integer in either decimal or hexadecimal format, or a label:

```
IMM 0x1f3d
IMM 257
IMM -5
IMM message
```

When an argument is a label, sembler will replace the label with the address that label represents. For a list of all instructions, refer to the Stockfighter Virtual Machine documentation.

The following directives are implemented:

* `.asciz`: This directive takes a string as an argument, and instructs the assembler to write that string to memory followed by a zero byte. For example, `.asciz "foo"` will emit the bytes `66 6F 6F 00`. This type of string is also known as a C-style string or a null-terminated string.
* `.ascii`: Same as `.asciz`, but without a null byte at the end.
* `.db`: This directive takes a single byte as an argument, which it emits.
* `.dw`: This directive takes a word (32-bit integer), and emits it as four bytes, big-endian. For example, `.dw 0x12345678` will emit the bytes `12 34 56 78`.

## Example

```
bss {
  message: .asciz "Hello, world!"
}

raw {
factorial:
    ENT 0x0
    REL 0x0
    LI
    BNZ recurse
    IMM 0x1
    RET
recurse:
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
    ADJ 0x1
    IMM 0x0
    RET
}
```

Save this file as `test.svm`, and then run `sembler test.svm`. The output should be

```json
{
  "bss": "SGVsbG8sIHdvcmxkIQA=",
  "ep": 55,
  "ok": true,
  "raw": "CgAAAAACAAAAAA0JAAAAFgUAAAABDAIAAAAADREFAAAAAR40BwAAAAALAAAAARECAAAAAA0fDAoAAAAABQAAAAA0IgAAAAILAAAAAQUAAAAHNAcAAAAACwAAAAERIgAAAAELAAAAAQUAAAAADA=="
}
```

If you run this in the Stockfighter Virtual Machine, the output should be

```
Hello, world!
5040
```

You can find more examples in the `examples` directory.

## Building

Sembler is written in Rust. Since there currently are no prebuilt binaries, you'll have to build it yourself. Make sure Rust is installed (go to https://www.rust-lang.org/ or use your preferred package manager if it is not), clone this project, and run `cargo build --release` (use `cargo build` to build a debug if you want to contribute). You'll be able to find the `sembler` executable in the `target/release` directory.

## Unimplemented features

This is a wishlist of features that I'd like or am planning to add. If you would like to implement any of these, please create an issue for the feature and mention that you're implementing it yourself.

- [ ] Useful error handling and messages. Currently, there are a lot of panics in the code, and error messages often don't specify what line of the input caused them.
- [ ] Fancier expressions. Instruction arguments are either literals or labels at the moment. It would be nice to allow arithmetic that is evaluated to a constant at assembly time. For example, 'IMM 3 * 60 * 60' or 'IMM some_record + 5'.
- [ ] Macros. Macros can give you some of the abstraction and code reuse features of higher level languages.
- [ ] Object file output. I'd like sembler to be able to output object files without an entry point containing functions, which could then be linked into a final executable by a linker.
- [ ] Tests. Rust has a built-in testing framework; which I'm currently not using. I'd like to remedy that.
