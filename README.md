# Sscanf in RUST using macros

![Language](https://img.shields.io/badge/Language-RUST-critical?style=for-the-badge&logo=rust)
![System](https://img.shields.io/badge/System-Arch_WSL2-A100FF?style=for-the-badge&logo=windows)

Rust implementation of C function sscanf. It is not entirely equal, as it receives a string and a array of separators, instead of a formatted string with types.

>Not using any extern crates, but unstable feature [**declarative macros 2.0**](https://github.com/rust-lang/rust/issues/39412). 

>Using macro_rules! works fine and will compile on stable. 

# Usage

```rust
let a :u8;   
let b :i32;   
let c :String;   
let d :f32;

let buffer = "02:98;abc,39.6";
let sep = [':',';',',',' '];

scan::sscanf!(buffer,sep,[u8,i32,String,f32],a,b,e,d);

println!("{} {} {} {}",a,b,e,d);
````
Result
```
2 98 abc 39.6
````

# Notes

- Using **nightly-x86_64-unknown-linux-gnu**.
- Using Macro 2.0 instead of macro_rules! , as it seems to be getting deprecated, but works in both situations.
- Using #![feature(decl_macro)]