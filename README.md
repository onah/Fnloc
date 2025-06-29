# Fnloc

**Fnloc** is a fast and minimal command-line tool to count lines of code per function in Rust source files.  
It helps you identify large or bloated functions by measuring their size and structure.

## Features

- Counts total lines, code lines, comment lines, and empty lines per function
- Supports:
  - Free functions (`fn foo()`)
  - Methods inside `impl` blocks (`impl Foo { fn bar(&self) {} }`)
- Output sorted by largest function first
- Fast and dependency-light (uses [`syn`](https://docs.rs/syn) internally for parsing)

## Lisence

MIT Lisense

