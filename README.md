# Stack Language Interpreter

A simple stack-based programming language interpreter written in Rust.

## Overview

This is a minimalist stack-based language similar to Forth or PostScript. Programs manipulate a stack of 32-bit integers and can store values in named variables.

## Installation

```bash
cargo build --release
```

## Usage

```bash
cargo run <program_file>
```

Example:

```bash
cargo run examples/countdown.emu
```

## Language Reference

### Stack Operations

- **PUSH <value|variable>** - Push a number or variable value onto the stack

```emu
  PUSH 42        # Push literal 42
  PUSH x         # Push value of variable x
```

- **POP** - Pop and print the top value from the stack

```emu
  PUSH 5
  POP
```

### Arithmetic Operations

- **ADD** - Pop two values, push their sum

```emu
  PUSH 3
  PUSH 4
  ADD            # Stack: [7]
```

- **SUB** - Pop two values, push their difference (second - first)

```emu
  PUSH 10
  PUSH 3
  SUB            # Stack: [7]
```

### Variables

- **ASSIGN <name>** - Pop top value and store it in a variable

```emu
  PUSH 42
  ASSIGN x       # x = 42
```

- **VAR <name>** - Print the value of a variable

```emu
  VAR x          # Prints: 42
```

### Control Flow

- **LABEL <name>** - Mark a position in the program for jumping

```emu
  LABEL loop_start
```

- **JNZ <label>** - Pop a value; if non-zero, jump to the label

```emu
  PUSH 1
  JNZ loop_start # Jumps if top of stack != 0
```

### Comparison Operations

- **GZ** - Pop a value; push 1 if > 0, otherwise push 0

```emu
  PUSH 5
  GZ             # Stack: [1]
```

### Output

- **PRINT <text>** - Print literal text

```emu
  PRINT Hello    # Prints: "Hello"
```

## Example Programs

### Countdown from 5

```emu
PUSH 5
LABEL loop
ASSIGN counter
VAR counter # Prints: counter 
PUSH counter
PUSH 1
SUB
JNZ loop
```

### Simple Calculator

```emu
PUSH 10
PUSH 5
ADD
POP            # Prints: 15

PUSH 20
PUSH 7
SUB
POP            # Prints: 13
```

### Using Variables

```emu
PUSH 42
ASSIGN x
PUSH 8
ASSIGN y
PUSH x
PUSH y
ADD
POP            # Prints: 50
```

### More complex

```emu
PUSH 1
PUSH 2
ASSIGN x 
ASSIGN y
PUSH 10
LABEL fib
ASSIGN counter
PUSH y
PUSH x
ADD
PUSH x
ASSIGN y
ASSIGN x
PUSH counter
PUSH 1
SUB
JNZ fib
PRINT RESULT
VAR x
```

## Error Handling

The interpreter provides detailed error messages with line numbers:

```emu
ERROR: Invalid PUSH operation at LINE: 5: Variable does not exist
ERROR: Invalid POP operation at LINE: 3: Empty stack
```

## Limitations

- Only supports 32-bit signed integers
- No floating-point arithmetic
- No string manipulation
- No file I/O operations
- No functions/subroutines
