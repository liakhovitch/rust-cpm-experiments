# rust-z80 playground/example
This repo exists purely so I can share minimal examples of Rust programs written for the z80 CPU and CP/M operating system.
I plan to eventually publish a proper-ish library for interfacing with CP/M.
Many thanks to the [rust-z80](https://github.com/llvm-z80/rust-z80) and [llvm-z80](https://github.com/llvm-z80/llvm-z80) projects.

## Current example
CP/M syscalls have a tendency to modify all of the Z80's general-purpose registers, without saving and restoring prior CPU state. I am implementing syscall wrappers as inline assembly blocks, marking all general-purpose registers as outputs to prevent rustc from relying on them. This repository contains multiple implementations of the "write character to console" syscall, and print functions which call the syscall in a loop.
The most natural implementation produces sub-optimal code, so I included some alternate implementations -- which currently work better, but each have drawbacks.

Files of note:
* .cargo/config.toml contains various compiler and linker settings:
    * Release and dev builds are defined the same
    * Symbols are kept in, since objdump will throw them away anyway
    * We optimize for size, but set LLVM's inline threshold manually to encourage inlining
* layout.ld is the custom linker script
    * CP/M loads user programs at address 0x100
* disassembly.txt contains a recent disassembly output, for convenience
* And of course, src/main.rs

