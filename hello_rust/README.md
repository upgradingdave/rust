# Rust

## Resources

- https://doc.rust-lang.org/rust-by-example/
- https://rustlings.rust-lang.org/

# Installation

Install using `rustup`: 

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install a c compiler: 

```
xcode-select --install
```

## Uninstall Rust

```
rustup self uninstall
```

## Updating Rust

Update to a newly released version: 

```
rustup update
```

# Getting Started

- Run `rustup doc --book` to open the Rust book in your browser.
- Run `rustup doc` to open the Rust documentation in your browser.

# Coding in Rust

- [Crates.io](https://crates.io) is the repository of Rust packages
- Run `crate update --verbose` to see latest dependencies
- Run `cargo doc --open` to open the Rust documentation in your browser.
- To pass args to cargo use `cargo run -- <args>`

# Concepts

## Ownership, aka, Resource Acquisition Is Initialization (RAII)

- Each value has an Owner
- there can only be one owner at a time
- when the owner goes out of scope, the value will be dropped

Instead of shallow copying, values are moved. References allow use of a value without taking ownership. 


