# EZSharp Compiler
EZSharp is a Java-like language originally created by Professor Eugene Zima at Wilfrid Laurier University. This compiler is a project for the CP471 course at WLU. The compiler is written in Rust to take advantage of its performance and safety features.

## Features
These are the necessary features needed for the project to be completed. They are updated as the project progresses.

1. ✅ Lexical Analysis
2. ✅ Syntax Analysis
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
Or you can run the compiled binary for the current release directly (tested on Windows 11 x86-64 only):
```bash
./ezsharp_compiler.exe <path-to-ezsharp-file>
```

Currently, the compiler only outputs the tokens found during Lexical Analysis and the symbols found during Syntax Analysis.
- The outputs are logged to a file called `tokens.log` and `symbol_table.log` respectively in a directory called `logs` in the root of the project.
- Any errors found during Lexical Analysis are also logged to a file called `lexical_errors.log` in the same directory.
- Any errors found during Syntax Analysis are also logged to a file called `syntax_errors.log` in the same directory.

This directory can be changed by providing the `--log-folder` option to the compiler:
```bash
cargo run -- <path-to-ezsharp-file> --log-folder <path-to-log-folder>
```

## Examples
The `test_programs` directory contains some sample EZSharp programs that can be used to test the compiler.

Given the following EZSharp program in a file called `/test_programs/Test10.cp`:
```
def int gcd(int a, int b)
    if (a == b) then
        return (a)
    fi;
    if (a > b) then
        return(gcd(a - b, b))
    else
        return(gcd(a, b - a))
    fi;
fed;
print gcd(21, 15);
print 45;
print 2 * (gcd(21, 28) + 6).
```

It can be compiled by running the following command in the root directory of the project:
```bash
cargo run -- test_programs/Test0.cp
```

Which outputs the following tokens to the `tokens.log` file:
```
Kdef on line 1
Kint on line 1
Identifier("gcd") on line 1
Soparen on line 1
Kint on line 1
Identifier("a") on line 1
Scomma on line 1
Kint on line 1
Identifier("b") on line 1
Scparen on line 1
...
Scomma on line 13
Tint(28) on line 13
Scparen on line 13
Oplus on line 13
Tint(6) on line 13
Scparen on line 13
Speriod on line 13
```

And outputs the following symbols to the `symbol_table.log` file:
```
Global {
	Func int gcd;
	Parameters {
		int a;
		int b;
	}
	Local {
	}
}
```

## Future Improvements
- Syntax Analysis
    - Automate First and Follow sets generation
    - Give better error messages (using empty cells in LL(1) table)
    - Clean up symbol table creation
    - Add support for parantheses in boolean expressions
    - Add support for negated expressions (grammar change)

## Additional Notes
- The Productions for this grammar and the First and Follow sets were generated manually and can be found in the `simplified_productions.txt` and `first_follow_set.txt` files respectively.
- The LL(1) table is generated automatically using the First and Follow sets, but a copy of what it looks like can be found in the `LL1_table.csv` file.
    - The production indices are the same as the indices for the productions found in the `syntax_analysis/productions.rs` file.
    - Production index 66 is outside of the array bounds as it represents all productions that go to epsilon.