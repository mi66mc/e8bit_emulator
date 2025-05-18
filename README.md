# e8Bit Emulator

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/e8bit_emulator)](https://crates.io/crates/e8bit_emulator)
[![Project GitHub](https://img.shields.io/badge/e8bit_emulator-github-green)](https://github.com/mi66mc/e8bit_emulator)
[![mi66mc](https://img.shields.io/badge/mi66mc-github-blue)]([https://](https://github.com/mi66mc))
</div>

This project is a simple 8-bit virtual machine (VM) emulator written in Rust. It simulates a basic CPU with registers, memory, and a set of instructions for performing arithmetic, memory operations, and control flow.

## Features

- **Registers**: Five general-purpose 8-bit registers (A, B, C, D, E).
- **Memory**: 256 bytes of memory.
- **Instruction Set**:
  - Arithmetic: `ADD`, `SUB`, `MUL`, `DIV`, `MOD`, `MULH`
  - Data Movement: `MOV`, `STORE`
  - Memory Access: Supports `[0]`, `[A]`, `[B]`, etc.
  - Control Flow: `JMP`, `JZ`, `JNZ`, `LOOP`
  - Input/Output: `INPUT`, `PRINT`, `PRINTCH`
  - Program Termination: `HALT`
  - Screen Operations: `DRAW`, `CLS`, `RENDER`
  - Comparison: `CMP`
- **Zero Flag**: Indicates whether the result of the last operation is zero, often used for conditional branching or logical evaluations. Comparisons evaluate to `false` (non-zero) or `true` (zero), enabling conditional logic.
- **Custom Parsing**: Accepts comments (`//`) and instruction separation via `;` or by lines.
- **Character Literals**: Supports character literals in instructions, e.g., `MOV A 'p'`. Characters are internally treated as their ASCII numeric values and must fit within 8 bits (0–255), just like any other number.
- **Debug Mode**: Optional debug mode for detailed output during execution.
- **IDLE Mode**: Allows direct input of instructions for testing and debugging.
- **Screen Rendering**: Supports drawing characters on an `80` by `25` virtual screen and rendering it to the console.
- **Label Support**: You can now use labels for control flow instructions (`JMP`, `JZ`, `JNZ`, `LOOP`) instead of numeric instruction indices.

## Label Support

You can define a label by writing it at the start of a line followed by a colon, e.g. `LOOP_START:`.  
You can then use the label name as the target for `JMP`, `JZ`, `JNZ`, or `LOOP` instructions:

```plaintext
MOV A 0
MOV B 10
LOOP_START:
ADD A 1
PRINT A
CMP A B
JNZ LOOP_START
HALT
```

This is equivalent to using numeric indices, but is easier to read and maintain.

## Example Programs

The files [`example.e8`](/examples/example.e8), [`example2.e8`](/examples/example2.e8), [`example3.e8`](/examples/example3.e8), [`example4.e8`](/examples/example4.e8), [`example5.e8`](/examples/example5.e8), [`example6.e8`](/examples/example6.e8), [`example7.e8`](/examples/example7.e8), and others contain example programs that demonstrate the use of registers, arithmetic operations, memory storage, loops, and conditional logic.

**Even or Odd Example:**
```plaintext
// EVEN OR ODD

INPUT A     // get number from user input
MOD A 2     // get remainder of A divided by 2
CMP A 0     // compare A with 0, if true, zero flag set to true
JZ EVEN     // EVEN (jump to label EVEN if zero)

MOV C 'O'   // ODD
PRINTCH C -N
MOV C 'D'
PRINTCH C -N
PRINTCH C -N
JMP END

EVEN:
MOV C 'E'   // EVEN
PRINTCH C -N
MOV C 'V'
PRINTCH C -N
MOV C 'E'
PRINTCH C -N
MOV C 'N'
PRINTCH C -N

END:
HALT        // end program
```

**Factorial Example (`example.e8`):**
```plaintext
// FACTORIAL SCRIPT

// registers
MOV A 1;           // A = 1 (result)
MOV B 5;           // B = 5 (number to get factorial)

LOOP_START:
MUL A B;           // A *= B
SUB B 1;           // B -= 1
LOOP LOOP_START B; // if B != 0 go to LOOP_START

// END
PRINT A;           // shows result
HALT;
```

## How to Run

1. **Install Rust**: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
2. **Compile the Program**:
   ```bash
   cargo build --release
   ```
3. **Run the Emulator**:
   ```bash
   cargo run example.e8 -d
   ```
   Replace `example.e8` with the path to your program file.

   If no file is specified, it will run in IDLE mode, where you can enter instructions directly.

   The `-d` flag is optional and enables debug mode, which provides additional output for debugging purposes.

## How to Write Programs

Programs for the emulator are written in a custom assembly-like language. Each instruction is written on a new line or separated by a semicolon and can include comments starting with `//`. Refer to the example programs above for syntax.

### Instruction Set

| Instruction      | Description                                                      |
| ---------------- | ---------------------------------------------------------------- |
| `MOV A 10`       | Move value 10 into register A                                    |
| `MOV A 'p'`      | Move character literal 'p' into register A (ASCII value)         |
| `MOV A [0]`      | Move value from memory address 0 into A                          |
| `MOV A [B]`      | Move value from memory at index stored in B                      |
| `ADD A B`        | A = A + B                                                        |
| `SUB A 1`        | A = A - 1                                                        |
| `MUL A 2`        | A = A * 2                                                        |
| `DIV A 2`        | A = A / 2                                                        |
| `MOD A 2`        | A = A % 2 (remainder after division)                             |
| `MULH A B C`     | A = high byte of (B * C)                                         |
| `STORE A [0]`    | Store A into memory\[0]                                          |
| `STORE A [B]`    | Store A into memory at index in B                                |
| `INPUT A`        | Read input (u8 or char) into register A                          |
| `JMP 10` / `JMP LABEL` | Jump to instruction index 10 or to label `LABEL`           |
| `JZ 5` / `JZ LABEL`    | Jump to index 5 or label if last result was 0 (zero flag set) |
| `JNZ 8` / `JNZ LABEL`  | Jump if last result was not zero (zero flag not set)       |
| `LOOP 3 C` / `LOOP LABEL C` | Jump to index 3 or label while C != 0                 |
| `PRINT A`        | Print value of A with newline                                    |
| `PRINT A -N`     | Print value of A without newline                                 |
| `PRINTCH A`      | Print character represented by value in A                        |
| `PRINTCH A -N`   | Print character without newline                                  |
| `DRAW X Y C`     | Draw character `C` at screen position `(X, Y)`                   |
| `CLS`            | Clear the screen                                                 |
| `CTS`            | Clear the terminal screen                                        |
| `RENDER`         | Render the screen to the console (80x25)                         |
| `SLP 1000`       | Pause execution for 1 second (1000 ms)                           |
| `HALT`           | Stops program execution                                          |
| `CMP A 10`       | Compare register A with value 10. Sets the zero flag if equal.   |

## Args Types

| Type            | Example     | Description                             |
| --------------- | ----------- | --------------------------------------- |
| Register        | `A`, `B`, `C`, `D`, `E` | One of the five registers               |
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
| `MOD`       | Register                            | Immediate Value, Register, or Memory Address                      | -                  |
| `MULH`      | Register                            | Register                                                          | Register           |
| `STORE`     | Register                            | Memory Address (`[0]`, `[B]`, etc.)                               | -                  |
| `JMP`       | Immediate Value                     | -                                                                 | -                  |
| `JZ`        | Immediate Value                     | -                                                                 | -                  |
| `JNZ`       | Immediate Value                     | -                                                                 | -                  |
| `LOOP`      | Immediate Value (Instruction Index) | Register                                                          | -                  |
| `PRINT`     | Register                            | *Optional*: `-N` to suppress newline                              | -                  |
| `PRINTCH`   | Register                            | *Optional*: `-N` to suppress newline                              | -                  |
| `INPUT`     | Register                            | -                                                                 | -                  |
| `DRAW`      | Immediate Value, Register, or Memory Address | Immediate Value, Register, or Memory Address | Immediate Value, Register, or Memory Address            |
| `CLS`       | -                                   | -                                                                 | -                  |
| `CTS`       | -                                   | -                                                                 | -                  |
| `RENDER`    | -                                   | -                                                                 | -                  |
| `SLP`       | Milliseconds                        | -                                                                 | -                  |
| `HALT`      | -                                   | -                                                                 | -                  |
| `CMP`       | Register                            | Immediate Value, Register, or Memory Address                      | -                  |

## Tips

- Use the `CTS` instruction to clear the terminal screen and `CLS` to clear virtual screen.
- Use the `MOD` instruction to easily check for even/odd numbers or perform modular arithmetic.
- The `CMP` instruction is useful for conditional branching with `JZ` and `JNZ`.
- Use `PRINTCH` for ASCII output and `PRINT` for numeric output.
- The virtual screen is 80 columns by 25 rows; use `DRAW`, `CLS`, and `RENDER` for simple graphics.

## Future Improvements

- Add support for more instructions.
- Implement debugging tools.
- Enhance error handling for invalid programs.

## License

This project is open-source and available under the MIT License.
