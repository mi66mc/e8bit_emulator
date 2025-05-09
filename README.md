# e8Bit Emulator

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/e8bit_emulator)](https://crates.io/crates/e8bit_emulator)
[![Project GitHub](https://img.shields.io/badge/e8bit_emulator-github-green)](https://github.com/mi66mc/e8bit_emulator)
[![mi66mc](https://img.shields.io/badge/mi66mc-github-blue)]([https://](https://github.com/mi66mc))
</div>

This project is a simple 8-bit virtual machine (VM) emulator written in Rust. It simulates a basic CPU with registers, memory, and a set of instructions for performing arithmetic, memory operations, and control flow.

## Features

- **Registers**: Four general-purpose 8-bit registers (A, B, C, D).
- **Memory**: 256 bytes of memory.
- **Instruction Set**:
  - Arithmetic: `ADD`, `SUB`, `MUL`, `DIV`, `MULH`
  - Data Movement: `MOV`, `STORE`
  - Memory Access: Supports `[0]`, `[A]`, `[B]`, etc.
  - Control Flow: `JMP`, `JZ`, `JNZ`, `LOOP`
  - Input/Output: `INPUT`, `PRINT`, `PRINTCH`
  - Program Termination: `HALT`
- **Zero Flag**: Tracks whether the result of the last operation was zero.
- **Custom Parsing**: Accepts comments (`//`) and instruction separation via `;` or by lines.
- **Character Literals**: Supports character literals in instructions, e.g., `MOV A 'p'`. Characters are internally treated as their ASCII numeric values and must fit within 8 bits (0–255), just like any other number.

## Example Program

The files `example.e8`, `example2.e8`, and `example3.e8` contain examples of programs that demonstrate the use of registers, arithmetic operations, memory storage, and loops.

```plaintext
// FACTORIAL SCRIPT

// registers
MOV A 1;           // A = 1 (result)
MOV B 5;           // B = 5 (number to get factorial)

// loop starts here (index 2)
MUL A B;           // A *= B
SUB B 1;           // B -= 1
LOOP 2 B;           // if B != 0 go to index 2 (MUL A B)

// END
PRINT A;           // shows result
HALT;
```

```plaintext
// WELCOME SCRIPT

MOV C 0;

INPUT A;

STORE A [C];
ADD C 1;
INPUT A;
JNZ 2;          // If nothing in the input continue, else go back

// Welcome message

MOV D 'H';       // H
PRINTCH D -N;
MOV D 'e';      // e
PRINTCH D -N;
MOV D 'l';      // l
PRINTCH D -N;
PRINTCH D -N;
MOV D 111;      // o
PRINTCH D -N;
MOV D ',';       // ,
PRINTCH D -N;
MOV D 32;       //  
PRINTCH D -N;

// Hello, 

MOV D 0;
MOV A [D];
PRINTCH A -N;   // No line break
ADD D 1;
MOV B C;
SUB B D;
JNZ 20;

// Hello, (name)

HALT
```

```plaintext
// "16" bits factorial

MOV A 0        // A = 0 (high byte)
MOV B 1        // B = 1 (low byte)
MOV C 6        // C = 6 (or 2, 5, etc.)
MOV D B        // D = B
MUL D C        // D = B * C (low 8 bits)
STORE D [0]    // Store low byte
MULH D B C     // D = B * C (high 8 bits)
STORE D [1]    // Store high byte
MOV D A        // D = A
MUL D C        // D = A * C (low 8 bits)
ADD D [1]      // D = (A * C) + high byte of B * C
MOV A D        // A = new high byte
MOV B [0]      // B = new low byte
SUB C 1        // C--
LOOP 3 C       // Loop to instruction 3 if C != 0


// if 6 (just examples to know how to calculate)
PRINT A        // 2
PRINT B        // 208
// 256 * 2 = 512
// 512 + 208 = 720
// 6! = 720


HALT
```

## How to Run

1. **Install Rust**: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
2. **Compile the Program**:
   ```bash
   cargo build --release
   ```
3. **Run the Emulator**:
   ```bash
   cargo run -- example.e8
   ```
   Replace `example.e8` with the path to your program file.

## How to Write Programs

Programs for the emulator are written in a custom assembly-like language. Each instruction is written on a new line and can include comments starting with `//`. Refer to the example program above for syntax.

### Instruction Set

| Instruction    | Description                                 |
| -------------- | ------------------------------------------- |
| `MOV A 10`     | Move value 10 into register A               |
| `MOV A 'p'`    | Move character literal 'p' into register A (treated as its ASCII value)  |
| `MOV A [0]`    | Move value from memory address 0 into A     |
| `MOV A [B]`    | Move value from memory at index stored in B |
| `ADD A B`      | A = A + B                                   |
| `SUB A 1`      | A = A - 1                                   |
| `MUL A 2`      | A = A \* 2                                  |
| `DIV A 2`      | A = A / 2                                  |
| `MULH A B C`   | A = high byte of (B * C)                    |
| `STORE A [0]`  | Store A into memory\[0]                     |
| `STORE A [B]`  | Store A into memory at index in B           |
| `INPUT A`      | Read input (u8 or char) into register A     |
| `JMP 10`       | Jump to instruction index 10                |
| `JZ 5`         | Jump to index 5 if last result was 0        |
| `JNZ 8`        | Jump if last result was not zero            |
| `LOOP 3 C`     | Jump to index 3 while C != 0                |
| `PRINT A`      | Print value of A with newline               |
| `PRINT A -N`   | Print value of A without newline            |
| `PRINTCH A`    | Print character represented by value in A   |
| `PRINTCH A -N` | Print character without newline             |
| `HALT`         | Stops program execution                     |

## Args Types

| Type            | Example     | Description                             |
| --------------- | ----------- | --------------------------------------- |
| Register        | `A`, `B`    | One of the four registers               |
| Immediate Value | `42`, `'p'` | A literal number or character between 0–255          |
| Memory Address  | `[0]`       | Direct access to memory index 0         |
| Memory via Reg  | `[A]`       | Access memory using value in register A |
> Note: Square brackets (`[]`) are used to specify memory addresses. For example:
> - `MOV A [0]` loads the value from memory address 0 into register A.
> - `STORE A [0]` stores the value of register A into memory address 0.

## Args Types Supported

| Instruction | Arg 1 Type                          | Arg 2 Type                                                        | Arg 3 Type         |
| ----------- | ----------------------------------- | ----------------------------------------------------------------- | ------------------ |
| `MOV`       | Register                            | Immediate Value, Register, or Memory Address (`[0]`, `[A]`, etc.) | -                  |
| `ADD`       | Register                            | Immediate Value, Register, or Memory Address                      | -                  |
| `SUB`       | Register                            | Immediate Value, Register, or Memory Address                      | -                  |
| `MUL`       | Register                            | Immediate Value, Register, or Memory Address                      | -                  |
| `DIV`       | Register                            | Immediate Value, Register, or Memory Address                      | -                  |
| `MULH`      | Register                            | Register                                                          | Register           |
| `STORE`     | Register                            | Memory Address (`[0]`, `[B]`, etc.)                               | -                  |
| `JMP`       | Immediate Value                     | -                                                                 | -                  |
| `JZ`        | Immediate Value                     | -                                                                 | -                  |
| `JNZ`       | Immediate Value                     | -                                                                 | -                  |
| `LOOP`      | Immediate Value (Instruction Index) | Register                                                          | -                  |
| `PRINT`     | Register                            | *Optional*: `-N` to suppress newline                              | -                  |
| `PRINTCH`   | Register                            | *Optional*: `-N` to suppress newline                              | -                  |
| `INPUT`     | Register                            | -                                                                 | -                  |
| `HALT`      | -                                   | -                                                                 | -                  |

## Future Improvements

- Add support for more instructions.
- Implement debugging tools.
- Enhance error handling for invalid programs.

## License

This project is open-source and available under the MIT License.
