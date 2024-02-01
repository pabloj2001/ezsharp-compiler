# EZSharp Compiler
EZSharp is a Java-like language originally created by Professor Eugene Zima at Wilfrid Laurier University. This compiler is a project for the CP471 course at WLU. The compiler is written in Rust to take advantage of its performance and safety features.

## Features
These are the necessary features needed for the project to be completed. They are updated as the project progresses.

1. ✅ Lexical Analysis
2. ⬜ Syntax Analysis
3. ⬜ Semantic Analysis
4. ⬜ Intermediary Code Generation

## Building
To build the project, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the project by running the following command in the root directory of the project:
```bash
cargo build
```
This will create a `target` directory in the root of the project with the compiled binary.

## Usage
To compile an EZSharp file, you can run the following command in the root directory of the project:
```bash
cargo run -- <path-to-ezsharp-file>
```
Or you can run the compiled binary for the current release directly:
```bash
./releases/ezsharp_compiler.exe <path-to-ezsharp-file>
```

Curently, the compiler only outputs the tokens found during Lexical Analysis. The output is logged to a file called `tokens.log` in a directory called `logs` in the root of the project. Any errors found during Lexical Analysis are also logged to a file called `lexical_errors.log` in the same directory.

This directory can be changed by providing the `--log-folder` option to the compiler:
```bash
cargo run -- <path-to-ezsharp-file> --log-folder <path-to-log-folder>
```

## Examples
The `test_programs` directory contains some example EZSharp programs that can be used to test the compiler.

Given the following EZSharp program in a file called `/test_programs/Test0.cp`:
```
// Hello world
int x = 23#3;.
```

It can be compiled by running the following command in the root directory of the project:
```bash
cargo run -- test_programs/Test1.cp
```

Which outputs the following tokens to the `tokens.log` file:
```
Kint
Identifier("x")
Oassign
Tint(23)
Tint(3)
Ssemicolon
Speriod
```

And it outputs the following errors to the `lexical_errors.log` file:
```
Invalid tokens:
InvalidToken { lexeme: "#", line: 2 }
```
