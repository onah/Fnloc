// This is a sample Rust file for testing
// Contains various functions with different line counts

/// A simple function with just one line of code
pub fn simple_function() -> i32 {
    42
}

/// A function with multiple lines and comments
pub fn complex_function(x: i32, y: i32) -> i32 {
    // First, we validate the inputs
    if x < 0 || y < 0 {
        return -1;
    }

    // Calculate the result
    let result = x * y + 10;

    // Apply some transformation
    let final_result = if result > 100 { result / 2 } else { result * 2 };

    final_result
}

/// An empty function for testing
pub fn empty_function() {}

/// A function with only comments
pub fn comment_heavy_function() {
    // This function demonstrates
    // multiple comment lines
    // without much actual code

    // Just a simple assignment
    let _x = 1;
}

/// A large function with mixed content
pub fn large_function(data: Vec<i32>) -> Vec<i32> {
    // Initialize result vector
    let mut result = Vec::new();

    // Process each element
    for item in data.iter() {
        // Check if item is positive
        if *item > 0 {
            // Add doubled value
            result.push(*item * 2);
        } else if *item < 0 {
            // Add absolute value
            result.push(item.abs());
        } else {
            // Skip zero values
            continue;
        }
    }

    // Sort the result
    result.sort();

    // Remove duplicates
    result.dedup();

    result
}
