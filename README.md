# Learning Rust

## Useful commands

- `cargo new project_name`: Create new project.

- `cargo init` (inside existing project directory): Initialize Cargo project in existing directory.

- `cargo build`: This command creates an executable file in target/debug/hello_cargo rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug.

- `cargo build --release`: This command will create an executable in target/release instead of target/debug. 

- `cargo run`: Build and run.

- `cargo check`: This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

- `cargo add rand`: add crate from `crates.io`, e.g. `rand`.

## Rust Resources

- [crates.io](https://crates.io/): Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.