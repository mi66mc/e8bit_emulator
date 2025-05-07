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
  - Arithmetic: `ADD`, `SUB`, `MUL`, `DIV`
  - Data Movement: `MOV`, `STORE`
  - Control Flow: `JMP`, `JZ`, `JNZ`, `LOOP`
  - Output: `PRINT`
  - Program Termination: `HALT`
- **Zero Flag**: Tracks whether the result of the last operation was zero.

## Example Program

The file `example.e8` contains an example program that demonstrates the use of registers, arithmetic operations, memory storage, and loops.

```plaintext
// Initialize registers
MOV A 10;          // Set register A to 10
MOV B 2;           // Set register B to 2
MOV C 5;           // Set register C to 5 (loop counter)

// Perform arithmetic operations
ADD A B;           // Add B to A (A = A + B)
SUB A 3;           // Subtract 3 from A (A = A - 3)
MUL B 4;           // Multiply B by 4 (B = B * 4)
DIV B 2;           // Divide B by 2 (B = B / 2)

// Store and load from memory
STORE A 0;         // Store the value of A into memory address 0
STORE B 1;         // Store the value of B into memory address 1
MOV D [0];         // Load the value from memory address 0 into register D
ADD D [1];         // Add the value from memory address 1 to D

// Print values
PRINT A;           // Print the value of register A
PRINT B;           // Print the value of register B
PRINT D;           // Print the value of register D

// Loop example
PRINT C;           // Print the current value of C
SUB C 1;           // Decrement C by 1
LOOP 14 C;         // Jump back to instruction 14, line 24 (may be confusing)

// Halt the program
HALT;              // Stop execution
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

| Instruction | Description                                                                 |
|-------------|-----------------------------------------------------------------------------|
| `MOV`       | Move a value into a register (`MOV A 10` or `MOV A [0]`).                  |
| `ADD`       | Add a value to a register (`ADD A B` or `ADD A 5`).                        |
| `SUB`       | Subtract a value from a register (`SUB A B` or `SUB A 3`).                 |
| `MUL`       | Multiply a register by a value (`MUL A B` or `MUL A 2`).                  |
| `DIV`       | Divide a register by a value (`DIV A B` or `DIV A 2`).                    |
| `STORE`     | Store a register's value into memory (`STORE A 0`).                        |
| `JMP`       | Jump to a specific instruction (`JMP 10`).                                |
| `JZ`        | Jump if the zero flag is set (`JZ 10`).                                   |
| `JNZ`       | Jump if the zero flag is not set (`JNZ 10`).                              |
| `LOOP`      | Decrement a register and jump if not zero (`LOOP 10 C`).                  |
| `PRINT`     | Print the value of a register (`PRINT A`).                                |
| `HALT`      | Stop program execution.                                                   |

## Args Types

- **Register**: A single character (A, B, C, D).
- **Immediate Value**: A number (e.g., 10, 5).
- **Memory Address**: A number in square brackets (e.g., `[0]`, `[1]`).
> Note: Square brackets (`[]`) are used only when specifying memory addresses for instructions other than `STORE`. For example:
> - `MOV A [0]` loads the value from memory address 0 into register A.
> - `STORE A 0` stores the value of register A into memory address 0 (no brackets required).

## Args Types Supported

| Instruction | Arg 1 Type | Arg 2 Type |
|-------------|------------|------------|
| `MOV`       | Register    | Immediate Value, Memory Address or Register |
| `ADD`       | Register    | Immediate Value, Memory Address or Register |
| `SUB`       | Register    | Immediate Value, Memory Address or Register |
| `MUL`       | Register    | Immediate Value, Memory Address or Register |
| `DIV`       | Register    | Immediate Value, Memory Address or Register |
| `STORE`     | Register    | Memory Address (Immediate Value in this case) |
| `JMP`       | Immediate Value | -          |
| `JZ`        | Immediate Value | -          |
| `JNZ`       | Immediate Value | -          |
| `LOOP`      | Immediate Value | Register    |
| `PRINT`     | Register    | -          |
| `HALT`      | -          | -          |

## Future Improvements

- Add support for more instructions.
- Implement debugging tools.
- Enhance error handling for invalid programs.

## License

This project is open-source and available under the MIT License.
