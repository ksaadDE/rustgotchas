# Rust Module Fuckery
Why are Rust modules so hard to understand? There are two options, either `Main.rs` or `Lib.rs`. And crates are not packages etc. 

The worst is, that if the main function in main.rs is missing (e.g. ure using rocket) and you try to run the main, it will not find the module main and avoid execution of that part of the program. Nice, right?

## How to use Rust Modules using `lib.rs`
If you want to use a top level module from a lower module directory like `src/bin` you will be using `lib.rs` and multiple binary crates. After doing so you can asically delete the `main.rs`, if it does not contain a `fn main`

In `lib.rs`, assuming you have two modules, a sub module in `src/bin/submodule.rs` and a a top level module `src/database.rs`:
```rust
  // lib.rs
  pub mod database;
```

In the `src/bin/submodule.rs` you can improt the module specified in lib.rs:
```rust
// src/bin/submodule.rs
  use projectname::database;
```

## How to run the new binary crates?
The (dis)advantage of multiple bin crates is obvious, you will need to execute them individually using `--bin` flag of the compiler / runner, e.g. `cargo run --bin <fname without rs>`

## How to import stuff globally using `lib.rs`?
Nice to know is also that you can import things like Rocket or Diesel globally using Lib.rs:
```rust
// lib.rs
// your consts here
// your pub mods here

// Rust 2018+ convention (no macro_use needed)
extern crate rocket;
extern crate rocket_dyn_templates; // optional
extern crate diesel;
```

## How to find the project name?
In `Cargo.toml` is the project name defined!
```toml
[package]
name = "project_name"
...
```
