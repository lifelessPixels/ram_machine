# ram_machine
RAM Machine emulator written in Rust

## Introduction
`ram_machine` is simple, very crude parser and emulator of RAM Machine ([wiki](https://en.wikipedia.org/wiki/Random-access_machine)) written in Rust programming language. It's implementation is based on description published by Łukasz Szkup [here](https://www.szkup.com/?pid=praca_mgr) (description is written in Polish, unfortuantely).

## Usage
```console
# in the main directory of repo:
cargo run [path to source code] <memory size (default: 512)>
```
You can also run example programs written in RASM (RAM assembly) provided in `examples\` directory of repo like that:
```console
cargo run examples/test.rasm    # or any other (yet not existant 😢) test program
```

If you want to see debug output (at the moment it is only printing executed instructions and their respective addresses) you can set environment variable `RAM_DEBUG` to any value.

## RAM Assembly syntax

### Comments

Comments are parts of souce code demoted by `;` which are ignored by emulator. They are terminated by skipping to new line (like `//` in C or simmilar languages). There is no notion for multi-line comments. They can be written like this:

```
; this is the comment
load 1 ; this is also the comment
```

### Labels

Labels can be introduced in a source code to notate a position to which the execution should be passed when `jump`, `jgtz`, `jzero` instruction jumps. Their syntax is very simple: `name_of_label:`. They can be written like this:

```
label1:
  ; do some stuff
  jump label1
```

### Instructions & operands

Emulator has a memory which can be used in the runtime of emulated program. Default memory size is `512` cells (each cell can be value fitting into i64 integer (-2^63 to 2^63-1)). Default memory size can be overriten by providing second argument containing desired memory size. Memory is 0-based indexed.

There is also notion of so called `accumulator` which lies in memory at address `0`. It is used by all the arithmetic instructions that this emulator supports. Direct memory access to accumulator can also be performed:

```
add 0 ; adds contents of accumulator to accumulator (double the accumulator)
```

There are 4 distinct operand types ([...] names are given to refer to these addressing modes :
* [`label`] - `label` - label name to which the instruction should jump to (NOTE: this name DOES NOT include the colon at the end)

```
label1: ; mark the place where to jump
  ; some stuff
jump label1 ; the jump occurs here
```

* [`imm`] - `=123` - immediate value, can be negative, denoted by `=` sign at front, evaluates to concrete value to be used in calculations

```
add =123 ; adds 123 to accumulator
```

* [`addr`] - `123` - immediate address value, cannot be negative, evaluates to the address from/to where in memory the value should be loaded/stored

```
add 123 ; adds contents of 123th cell of memory to accumulator
```

* [`iaddr`] - `^123` - intermediate address value, cannot be negative, address where the address of memory, that instruction will be operating on, will be loaded from (indirect addressing), the address lying at provided address also cannot be nagative (because it will be used as the actual address)

```
; imaginary memory contents: | 0 | 2 | 123 | ...
add ^1 ; (in this case) adds contents of memory[2] to accumulator (which is 123)
; basically it's saying:
;       accumulator += memory[memory[1]]
```

Instruction index:

| Instruction opcode | Valid operands         | Description                                                                                     |
|--------------------|------------------------|-------------------------------------------------------------------------------------------------|
| `load`             | `imm`, `addr`, `addri` | loads the value into accumulator                                                                |
| `store`            | `addr`, `addri`        | stores the value contained in accumulator in provided memory location                           |
| `add`              | `imm`, `addr`, `addri` | adds specified value to the accumulator                                                         |
| `sub`              | `imm`, `addr`, `addri` | subtracts specified value from the accumulator                                                  |
| `mult`             | `imm`, `addr`, `addri` | multiplies accumulator by specified value                                                       |
| `div`              | `imm`, `addr`, `addri` | divides accumulator by specified value                                                          |
| `read`             | `addr`, `addri`        | reads value from input and stores it in specified location in memory                            |
| `write`            | `imm`, `addr`, `addri` | writes specified value to output                                                                |
| `jump`             | `label`                | jumps unconditionally to specified label-denoted location                                       |
| `jgtz`             | `label`                | jump to specified label-denoted location if value contained in accumulator is greater than zero |
| `jzero`            | `label`                | jump to specified label-denoted location if value contained in accumulator is equal zero        |
| `halt`             | None                   | ends the execution of the program                                                               |
