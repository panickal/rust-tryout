# rust
Trying out rust sample programs.

# Key points
* strongly typed, can't assign from one type to another
* statically typed, checks type at compile-time
* variables are immutable by default, have to explicitly use `mut` keyword to make them mutable
* all memory accesses are checked
* strict ownership management
* array-bounds are checked
* code organized as
    * functions : similar to c++ functions
    * modules : similar to namespaces/files in c++, math can be a module
    * crates : similar to an executable/library/plugin/extension, json would be a crate

# Cargo
Cargo is the package manager for Rust.
* `cargo build` for building `Cargo.toml` which is the build config file
* `cargo check` for running tests
* `cargo run` for running project executable
* `cargo new name --bin` can be used for initializing a new binary crate or `--lib` for a new library crate
* cargo creates a target folder for temp and target files
