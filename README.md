# EZSharp Compiler
EZSharp is a Java-like language originally created by Professor Eugene Zima at Wilfrid Laurier University. This compiler is a project for the CP471 course at WLU. The compiler is written in Rust to take advantage of its performance and safety features.

## Features
These are the necessary features needed for the project to be completed. They are updated as the project progresses.

1. ✅ Lexical Analysis
2. ✅ Syntax Analysis
3. ✅ Semantic Analysis
4. ✅ Intermediary Code Generation

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

The compiler only outputs the tokens found during Lexical Analysis, the symbols found during Syntax Analysis, and the 3-TAC program:
- The outputs are logged to a file called `tokens.log`, `symbol_table.log`, and `tac` respectively in a directory called `logs` in the root of the project.
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
// Hello world
def int add(int x, int y)
    int z;
    return x + y;
fed;

int x, c, a[(5 > 3) * 3], b;
double d[1];
c = -23;
x = 3 + 2;

// while with brackets
while ((x + a[0]) < 3 and (x / 3 > 2)) do
    x = x + 1;
    if (x > 3) then
        print add(add(x, 3), 2);
    fi;
od;

a[1] = 2 + a[x * 3];.
```

It can be compiled by running the following command in the root directory of the project:
```bash
cargo run -- test_programs/Test0.cp
```

Which outputs the following tokens to the `tokens.log` file:
```
Kdef on line 2
Kint on line 2
Identifier("add") on line 2
Soparen on line 2
Kint on line 2
Identifier("x") on line 2
...
Identifier("x") on line 20
Omultiply on line 20
Tint(3) on line 20
Scbracket on line 20
Ssemicolon on line 20
Speriod on line 20
```

Outputs the following symbols to the `symbol_table.log` file:
```
{
	add: func(int, int) -> int
	{
		x: int
		y: int
		z: int
	}
	x: int
	c: int
	a: [int; 3]
	b: int
	d: [double; 1]
}
```

And outputs the 3-TAC codes to the `tac` file:
```
	Goto main0;
add1:
	BeginFunc 4;
	y1 = GetParams 4;
	x1 = GetParams 4;
	t0_ = x1 + y1;
	Return t0_;
	EndFunc;
...
	PopParams 4;
	Goto fi4;
fi4:
	Goto while0;
od1:
	t19_ = x0 * 3;
	*(a0 + 1) = 2 + *(a0 + t19_);
	EndFunc;
```

## Future Improvements
- Syntax Analysis
    - Automate First and Follow sets generation
    - Give better error messages (using empty cells in LL(1) table)
    - ~~Clean up symbol table creation~~
    - ~~Add support for parantheses in boolean expressions~~
    - ~~Add support for negated expressions (grammar change)~~

## Additional Notes
- The Productions for this grammar and the First and Follow sets were generated manually and can be found in the `simplified_productions.txt` and `first_follow_set.txt` files respectively.
- The LL(1) table is generated automatically using the First and Follow sets, but a copy of what it looks like can be found in the `LL1_table.csv` file.
    - The production indices are the same as the indices for the productions found in the `syntax_analysis/productions.rs` file.
    - Production index + 1 is outside of the array bounds as it represents all productions that go to epsilon.