# Fnloc Guides

This directory contains comprehensive documentation and guides for using the Fnloc tool effectively.

## Available Guides

### [Code Complexity Analysis Guide](complexity-analysis.md)
Comprehensive documentation covering:
- **Cyclomatic Complexity**: Measures the number of independent paths through code
- **Nesting Depth**: Measures how deeply nested control structures are
- **Usage Examples**: Practical examples and command-line techniques
- **Quality Assessment**: Guidelines for interpreting metrics
- **Refactoring Recommendations**: How to improve code based on metrics
- **CI/CD Integration**: Automated quality checks in build pipelines

## Quick Reference

### Complexity Guidelines
- **1-10**: Simple, easy to maintain
- **11-20**: Moderately complex, may need refactoring
- **21-50**: Complex, should be broken down
- **50+**: Very complex, high priority for refactoring

### Nesting Depth Guidelines
- **0-2**: Good, easy to follow
- **3-4**: Moderate, acceptable
- **5-6**: High, consider refactoring
- **7+**: Very high, strongly recommended to refactor

### Sample Output Format
```
  - fn function_name: total=X lines, code=Y, comment=Z, empty=W, complexity=C, nesting=N
```

## Contributing

When adding new guides:
1. Use clear, descriptive filenames
2. Include practical examples
3. Follow the existing documentation style
4. Update this README to reference new guides
