// Another test file with different function patterns

pub struct TestStruct {
    value: i32,
}

impl TestStruct {
    /// Constructor with minimal code
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// A method with moderate complexity
    pub fn process(&mut self, factor: i32) -> i32 {
        // Validate input
        if factor == 0 {
            return self.value;
        }

        // Update internal value
        self.value *= factor;

        // Return modified value
        self.value
    }

    /// A method with many comments but little code
    pub fn get_value(&self) -> i32 {
        // This method returns the current value
        // It's a simple getter method
        // No complex logic required
        // Just return the stored value
        self.value
    }
}

/// A standalone function with error handling
pub fn divide_numbers(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
