# A language that compiles to brainf*

Very incomplete and not much is implemented

Semicolons will break the code since the parser doesn't handle them yet.

Currently very limited

*Use `print()` to output expressions and input with read(N), where N is how many characters to read from input and must be a constant*

# Features

- [x] While loops
- [x] If statements
- [x] Variables
- [x] Input
- [x] Output
- [ ] Type system

# Supported operators

- [x] +
- [x] -
- [x] *
- [x] /
- [x] >
- [x] <
- [x] >=
- [x] <=
- [x] =
- [x] ==
- [x] !=

# Usage

You can use this as a compiler or an interpreter depending on the flags you run it with.
If running straight from cargo: `cargo run [input_file] -o [output_file] --interpret`

To view the help message: `cargo run -- --help` or `cargo run -- -h`, otherwise the flag will be handled by cargo instead of the program.

# Example

The example given by `example.bfs` takes an input of three characters and outputs a sequence from the first to the third character in a given direction.
For example `A>Z` would output the uppercase alphabet from A-Z and `Z<A` would output the alphabet in reverse.

The second example `extended_example.bfs` showcases the extension of brainfuck

# Brainfuck Extension

By passing `-e` or `--extended` you can enable an extended superset of brainfuck, which can interface with native code using just a single added instruction.

There are currently builtin interfaces for opening and writing to files.

# Technical Information

Internally, the code generation keeps track of the state of the code as it compiles.
However, issues arise from variables such as user input which can't be known at compile time.

In a completely pure program, the compiler will know the complete result and which paths will be taken, but just one use of input can contaminate an entire program.
The compiler does expose methods to read cells, but in a big script it's unlikely to be useful, since reading any cells that can't be known at compile time will error.


The compiler relies heavily on knowing the current cell position, and since whether or not a compiled BF loop will run or not might be impossible to predict, every compiled BF loop ***must*** end on the same position they started on. This ensures the positional state is always valid, since the end position will be the same regardless of if the loop actually runs or not.