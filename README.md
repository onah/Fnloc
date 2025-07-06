# Fnloc

**Fnloc** is a fast and minimal command-line tool to analyze Rust source code and measure function complexity.  
It counts lines of code per function and calculates cyclomatic complexity and nesting depth to help identify large or complex functions that may need refactoring.

## Features

- **Line counting**: Total lines, code lines, comment lines, and empty lines per function
- **Cyclomatic complexity**: Measures the number of independent paths through each function
- **Nesting depth**: Measures how deeply nested control structures are within functions
- **Comprehensive support**:
  - Free functions (`fn foo()`)
  - Methods inside `impl` blocks (`impl Foo { fn bar(&self) {} }`)
- **Smart output**: Results sorted by code lines (largest first) with complexity and nesting metrics
- **Unix-friendly**: Composable with standard command-line tools for filtering and analysis
- **Fast and reliable**: Uses [`syn`](https://docs.rs/syn) for accurate Rust AST parsing
- **Cargo integration**: Can be used as a Cargo subcommand (`cargo fnloc`)

## Installation

```bash
# Install as a Cargo subcommand
cargo install --path .

# Or build locally
cargo build --release
```

## Quick Start

```bash
# Using as a Cargo subcommand (recommended)
cargo fnloc                                    # Analyze current project's src directory
cargo fnloc path/to/your/rust/code            # Analyze specific directory
cargo fnloc --format json                     # Output in JSON format

# Or running directly
cargo run -- src                              # Analyze all functions in the src directory
cargo run -- path/to/your/rust/code          # Analyze a specific directory

# Example output:
#   - fn src\analyzer.rs::complex_function: total=42 lines, code=28, comment=8, empty=6, complexity=12, nesting=4
#   - fn src\utils.rs::simple_helper: total=5 lines, code=4, comment=1, empty=0, complexity=1, nesting=0
```

## Documentation

- **[Code Complexity Analysis Guide](guides/complexity-analysis.md)**: Comprehensive guide to cyclomatic complexity and nesting depth metrics
- **[Command Examples](COMMAND_EXAMPLES.md)**: Unix-style composition and filtering techniques

## License

MIT License

