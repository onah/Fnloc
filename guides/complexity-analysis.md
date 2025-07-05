# Code Complexity Analysis Guide

## Overview

The Fnloc tool provides comprehensive **code complexity analysis** for each function, measuring both **cyclomatic complexity** and **nesting depth** alongside line count analysis. These metrics provide deeper insights into code quality and maintainability.

## Cyclomatic Complexity

### What is Cyclomatic Complexity?

Cyclomatic complexity is a software metric that measures the number of linearly independent paths through a program's source code. It was developed by Thomas J. McCabe in 1976 and is widely used to assess code complexity.

### How it's Calculated

The calculation starts with a base complexity of 1 and adds 1 for each decision point in the code:

**Decision points in Rust include:**
- `if` and `else if` statements
- `match` expressions (each arm adds 1)
- `while` loops
- `for` loops  
- `loop` statements
- `return` statements (except the final one)
- `break` and `continue` statements
- `&&` and `||` operators in boolean expressions
- `?` operator (try expressions)
- Guard conditions in match arms

### Complexity Guidelines

- **1-10**: Simple, easy to understand and maintain
- **11-20**: Moderately complex, may need refactoring
- **21-50**: Complex, should be broken down into smaller functions
- **50+**: Very complex, high risk for bugs and difficult to maintain

## Nesting Depth

### What is Nesting Depth?

Nesting depth measures how deeply nested the control structures are within a function. Each level of `if`, `loop`, `match`, `block`, etc. increases the nesting depth. This metric helps identify functions that may be difficult to read and understand due to excessive nesting.

### How it's Calculated

The calculation starts at depth 0 (function level) and increases by 1 for each nested control structure:

**Structures that increase nesting depth:**
- `if`/`else if`/`else` blocks
- `match` expressions
- `for`/`while`/`loop` statements
- Block expressions `{}`
- Closures `|| {}`
- `async` blocks
- `unsafe` blocks

### Nesting Depth Guidelines

- **0-2**: Good, easy to follow
- **3-4**: Moderate, acceptable but watch for complexity
- **5-6**: High, consider refactoring to reduce nesting
- **7+**: Very high, strongly recommended to refactor

### Example of Nesting Analysis

```rust
fn example_function() {           // depth 0 (function level)
    if condition1 {               // depth 1
        for item in items {       // depth 2
            if condition2 {       // depth 3
                match value {     // depth 4
                    Some(x) => {  // depth 5
                        if x > 0 {// depth 6
                            // process
                        }
                    }
                    None => {}
                }
            }
        }
    }
}
// Maximum nesting depth: 6
```

## Usage Examples

### Basic Analysis

```bash
# Analyze all functions in the current src directory
cargo run -- src

# Analyze a specific directory
cargo run -- path/to/your/rust/code
```

### Sample Output

```
Analyzing 9 Rust files...

  - fn src\complexity_analyzer.rs::analyze_expression: total=186 lines, code=131, comment=23, empty=32, complexity=46, nesting=4
  - fn src\file_scanner.rs::find_rust_files: total=32 lines, code=28, comment=0, empty=4, complexity=8, nesting=5
  - fn src\function_extractor.rs::find_function_bounds: total=29 lines, code=25, comment=1, empty=3, complexity=13, nesting=6
  - fn src\function_analyzer.rs::analyze_function_lines: total=28 lines, code=24, comment=1, empty=3, complexity=5, nesting=4
  - fn src\main.rs::main: total=4 lines, code=4, comment=0, empty=0, complexity=1, nesting=0
```

### Unix-Style Filtering and Sorting

Since fnloc follows Unix philosophy, you can combine it with standard command-line tools:

```bash
# Show only functions with high complexity (>10)
cargo run -- src | grep "complexity=" | awk -F'complexity=' '{if($2+0 > 10) print}'

# Show only functions with high nesting (>4)
cargo run -- src | grep "nesting=" | awk -F'nesting=' '{if($2+0 > 4) print}'

# Sort by complexity (ascending)
cargo run -- src | grep "complexity=" | sort -t'=' -k6 -n

# Sort by nesting depth (ascending)
cargo run -- src | grep "nesting=" | sort -t'=' -k7 -n

# Count functions by complexity range
cargo run -- src | grep "complexity=" | awk -F'complexity=' '{
    if($2+0 <= 5) simple++; 
    else if($2+0 <= 10) moderate++; 
    else if($2+0 <= 20) complex++; 
    else very_complex++
} END {
    print "Simple (1-5):", simple+0; 
    print "Moderate (6-10):", moderate+0; 
    print "Complex (11-20):", complex+0; 
    print "Very Complex (20+):", very_complex+0
}'

# Count functions by nesting depth
cargo run -- src | grep "nesting=" | awk -F'nesting=' '{
    if($2+0 <= 2) shallow++; 
    else if($2+0 <= 4) moderate++; 
    else if($2+0 <= 6) deep++; 
    else very_deep++
} END {
    print "Shallow (0-2):", shallow+0; 
    print "Moderate (3-4):", moderate+0; 
    print "Deep (5-6):", deep+0; 
    print "Very Deep (7+):", very_deep+0
}'
```

## Code Quality Assessment

### Combined Metrics Analysis

The power of fnloc comes from analyzing both metrics together:

| Complexity | Nesting | Assessment | Action Required |
|------------|---------|------------|-----------------|
| Low (1-5) | Low (0-2) | ✅ Excellent | Maintain current quality |
| Low (1-5) | High (5+) | ⚠️ Deep but simple | Flatten structure |
| High (15+) | Low (0-2) | ⚠️ Many branches | Extract methods |
| High (15+) | High (5+) | ❌ Very complex | Priority refactoring |

### Example Analysis

Looking at the fnloc codebase itself:

```
  - fn analyze_expression: complexity=46, nesting=4    # High complexity, moderate nesting - should be refactored
  - fn find_function_bounds: complexity=13, nesting=6  # Moderate complexity, high nesting - reduce nesting
  - fn find_rust_files: complexity=8, nesting=5        # Good complexity, moderate nesting - acceptable
  - fn analyze_function_lines: complexity=5, nesting=4 # Good complexity, moderate nesting - good
  - fn main: complexity=1, nesting=0                   # Excellent - minimal complexity and nesting
```

### Refactoring Recommendations

**For High Cyclomatic Complexity:**
1. **Extract methods**: Break down large functions into smaller, focused functions
2. **Simplify conditions**: Break complex boolean expressions into well-named variables
3. **Use pattern matching**: Replace complex if-else chains with match expressions
4. **Early returns**: Use guard clauses to reduce branching

**For High Nesting Depth:**
1. **Extract nested blocks**: Move deeply nested code into separate functions
2. **Use early returns**: Reduce nesting with guard clauses
3. **Flatten conditionals**: Combine conditions or use logical operators
4. **Iterator methods**: Replace nested loops with iterator chains where appropriate

## Integration with Development Workflow

### CI/CD Integration

You can integrate complexity checks into your build process:

```bash
# Fail if any function has complexity > 20
if cargo run -- src | grep -q "complexity=[2-9][0-9]\\|complexity=[0-9][0-9][0-9]"; then
    echo "Functions with high complexity found!"
    exit 1
fi

# Fail if any function has nesting > 6
if cargo run -- src | grep -q "nesting=[7-9]\\|nesting=[0-9][0-9]"; then
    echo "Functions with high nesting depth found!"
    exit 1
fi
```

### Pre-commit Hooks

Use fnloc in pre-commit hooks to monitor complexity changes:

```bash
#!/bin/bash
# Check for high complexity or nesting
cargo run -- src | awk '
/complexity=/ {
    split($0, parts, "complexity=")
    if (parts[2]+0 > 15) print "High complexity:", $0
}
/nesting=/ {
    split($0, parts, "nesting=")
    if (parts[2]+0 > 5) print "High nesting:", $0
}'
```

### Code Review Guidelines

During code reviews, use both metrics as guides:

**Complexity Guidelines:**
- 1-5: Generally good, minimal review needed
- 6-10: Review logic flow, ensure clarity
- 11-20: Require thorough review, consider refactoring
- 20+: Strong recommendation to refactor before merging

**Nesting Guidelines:**
- 0-2: Excellent structure
- 3-4: Good, review for clarity
- 5-6: Consider flattening structure
- 7+: Requires refactoring before merging

## Technical Implementation

Both metrics are calculated using Rust's `syn` crate to parse the Abstract Syntax Tree (AST) of each function:

1. **Parses the source code** into an AST using `syn::parse_file`
2. **Traverses each function** using the AST structure
3. **Counts decision points** (complexity) and tracks nesting levels (depth)
4. **Calculates metrics** following standard methodologies

### Supported Rust Constructs

The implementation recognizes all major Rust control flow constructs:

- **Conditional expressions**: `if`, `if let`, ternary-like expressions
- **Pattern matching**: `match` expressions with all arm patterns
- **Loops**: `for`, `while`, `loop` with `break`/`continue`
- **Early returns**: explicit `return` statements
- **Error handling**: `?` operator for Result/Option unwrapping
- **Boolean logic**: `&&` and `||` operators
- **Guard conditions**: `if` guards in match arms
- **Block structures**: `{}`, `async {}`, `unsafe {}`
- **Closures**: `|| {}` and `move || {}`

### Accuracy and Limitations

The implementation provides accurate measurements for standard Rust code. Some limitations:

- **Macros**: Macro expansions are not analyzed (defaults to base values)
- **Closures**: Analyzed as separate entities when possible
- **Async constructs**: Basic support for async blocks and functions

## Examples from Real Code

### Simple Function (Complexity = 1, Nesting = 0)
```rust
pub fn read_rust_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}
```

### Moderate Complexity (Complexity = 5, Nesting = 2)
```rust
pub fn complex_function(x: i32, y: i32) -> i32 {
    if x < 0 || y < 0 {        // +1 complexity (if), +1 (||), nesting +1
        return -1;             // +1 complexity (return)
    }
    
    let result = x * y + 10;
    let final_result = if result > 100 { // +1 complexity (if), nesting +1
        result / 2 
    } else { 
        result * 2 
    };
    
    final_result
}
// Complexity: Base 1 + if 1 + || 1 + return 1 + if-expr 1 = 5
// Nesting: Maximum depth of nested structures = 2
```

### High Complexity and Nesting Function
```rust
pub fn process_data(data: Vec<i32>) -> Vec<i32> {        // depth 0
    let mut result = Vec::new();
    
    for item in data.iter() {                            // depth 1, +1 complexity
        if *item > 0 {                                   // depth 2, +1 complexity
            match item % 3 {                             // depth 3, +1 complexity
                0 => {                                   // depth 4, +1 complexity
                    if *item > 100 {                     // depth 5, +1 complexity
                        result.push(*item / 2);
                    } else {
                        result.push(*item * 2);
                    }
                }
                1 => result.push(*item + 1),             // +1 complexity
                _ => result.push(*item - 1),             // +1 complexity
            }
        } else if *item < 0 {                            // depth 2, +1 complexity
            result.push(item.abs());
        } else {
            continue;                                    // +1 complexity
        }
    }
    
    result
}
// Complexity: Base 1 + for 1 + if 1 + match 1 + 3 arms + inner if 1 + else if 1 + continue 1 = 10
// Nesting: Maximum depth = 5
```

## Contributing to Analysis

The complexity analysis implementation is extensible and follows Rust best practices:

- **Well-tested**: Comprehensive unit tests cover all expression types
- **Modular design**: Separate `complexity_analyzer` module
- **Documentation**: Extensive inline documentation
- **Error handling**: Graceful degradation when parsing fails

For contributing improvements or reporting issues with the analysis, see the main project documentation.
