# How to setup Rust, Docker and SQLite 3.35+?

## The problem
Sadly, the old buster version contains only SQLite3.34. It's a problem because you need for e.g. returning clauses the 3.35+ version. How to fix that?

## The solution
The default `DOCKERFILE` contains `FROM rust`, change it to `FROM rust:bookworm` and rebuild it, it will install the newest version of Debian. The newest version contains SQLite 3.35+ and your problem will be gone :)
