# Fnloc

**Fnloc** is a fast and minimal command-line tool to analyze Rust source code and measure function complexity.  
It counts lines of code per function and calculates cyclomatic complexity to help identify large or complex functions that may need refactoring.

## Features

- **Line counting**: Total lines, code lines, comment lines, and empty lines per function
- **Cyclomatic complexity**: Measures the number of independent paths through each function
- **Comprehensive support**:
  - Free functions (`fn foo()`)
  - Methods inside `impl` blocks (`impl Foo { fn bar(&self) {} }`)
- **Smart output**: Results sorted by code lines (largest first) with complexity metrics
- **Unix-friendly**: Composable with standard command-line tools for filtering and analysis
- **Fast and reliable**: Uses [`syn`](https://docs.rs/syn) for accurate Rust AST parsing

## Quick Start

```bash
# Analyze all functions in the src directory
cargo run -- src

# Analyze a specific directory
cargo run -- path/to/your/rust/code

# Example output:
#   - fn src\analyzer.rs::complex_function: total=42 lines, code=28, comment=8, empty=6, complexity=12
#   - fn src\utils.rs::simple_helper: total=5 lines, code=4, comment=1, empty=0, complexity=1
```

## Documentation

- **[Code Complexity Analysis Guide](guides/complexity-analysis.md)**: Comprehensive guide to cyclomatic complexity and nesting depth metrics
- **[Command Examples](COMMAND_EXAMPLES.md)**: Unix-style composition and filtering techniques

## License

MIT License

