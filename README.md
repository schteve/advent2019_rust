# Advent of Code 2019
This repository contains my solutions for the [Advent of Code 2019](https://adventofcode.com/2019) programming puzzles, written in Rust 🦀.

This was my first Advent of Code, started around the time the event was running "live".

I used this as an opportunity to learn the Rust language. I started this event having only written a few hundred lines of Rust code and wanted to get a firm grasp of the language with some bite-sized objectives. Also, I wanted to brush up on some Computer Science skills that I rarely use in my professional work, such as graph theory. Advent of Code was perfect for this! In the end, my code was likely ugly and inefficient but for a first attempt it was good enough. I will likely not come back to update my solutions any further.

My goals were:
1. Solve the puzzles in a reasonably robust way.
    * Create solutions that were "general enough", such that any reasonable input would be solved by the program. In some cases it would be prohibitively difficult to make a truly general solution and the input data showed clear intent to not require one, then some shortcuts were acceptable.
    * Create programs with good structure. Use structs, mods, and composition.
    * Follow best practices by writing unit tests wherever it benefited the process of solving the puzzle. Typically, any important functionality with non-trivial edge cases would be tested.
    * Documentation was not a priority. I made no effort to add comments describing the overall solution, nor the individual functions. Comments were added only where it helped the writing process. In a real life production environment comments and good documentation would of course be required.
2. Write idiomatic Rust code wherever possible.
    * The code compiles with no warnings, has no clippy warnings (see below), and conforms to the Rust formatting guidelines.
    * Use only features of the latest version of stable Rust.
    * No unsafe code.
3. Learn or practice features of the Rust language where the solutions present an opportunity, even if it's not the ideal fit for the situation.
4. Create efficient solutions. I usually chose good structure over optimization, but for days when the solution took a long time to run the optimization became a priority. At this point many solutions are nowhere near optimal and significant optimization could be done.
5. Use few external crates, relying mainly on the standard library.

# Building and running
This project uses the [Cargo AoC](https://github.com/gobanos/cargo-aoc) framework, which must be installed in order to build the program. Cargo AoC makes it easy to download input files and supply them to the program, separate generators and solvers, and execute solutions selectively. It also provides an easy way to benchmark solutions.

All solutions can be tested and run with the usual cargo commands:
* `cargo test`
* `cargo run --release`

The solutions can be selectively run as follows:
* `cargo aoc -d D`, where D is replaced with the relevant day number (1-25)
* `cargo aoc -d D -p P`, same as above but replacing P with the relevant part number (1-2)

## Clippy
The clippy linter does not produce any warnings on the code at the default warning levels, with few exceptions where it is suppressed:
* `clippy::bool_comparison` and `clippy::needless_bool` - I find it far more readable to explicitly write booleans in most places they are used

## Commit hook
Each commit is checked with the following commands:
* `cargo fmt -- --check`
* `cargo test`
* `cargo clean -p advent_of_code_2019; cargo clippy -- -Dwarnings`

# Execution times
TBD
