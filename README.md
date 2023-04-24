# Learning Rust

## Useful commands

- `cargo new project_name`: Create new project.

- `cargo init` (inside existing project directory): Initialize Cargo project in existing directory.

- `cargo build`: This command creates an executable file in target/debug/hello_cargo rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug.

- `cargo build --release`: This command will create an executable in target/release instead of target/debug. 

- `cargo run`: Build and run.

- `cargo check`: This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

- `cargo add rand`: add crate from `crates.io`, e.g. `rand`.

- 🧨`cargo doc --open`: Build documentation provided by all your dependencies locally and open it in your browser.

## Rust Resources

- [crates.io](https://crates.io/): Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

## Rust Feats

- Rust has a strong, static type system. However, it also has type inference.

- A `match` expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the `match` construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all.
