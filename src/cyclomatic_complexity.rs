use syn::{Arm, Block, Expr, Item, ItemFn, Stmt};

/// Calculates the cyclomatic complexity of a function
///
/// Cyclomatic complexity is a software metric that measures the number of linearly
/// independent paths through a program's source code. It starts with a base complexity
/// of 1 and adds 1 for each decision point in the code.
///
/// Decision points in Rust include:
/// - if/else if statements
/// - match expressions (each arm adds 1)
/// - while loops
/// - for loops
/// - loop statements
/// - return statements (except the final one)
/// - break/continue statements
/// - && and || operators in boolean expressions
/// - ? operator (try expressions)
pub fn calculate_cyclomatic_complexity(func: &ItemFn) -> usize {
    let mut complexity = 1; // Base complexity

    // Analyze the function body
    complexity += analyze_block(&func.block);

    complexity
}

/// Analyzes a block of statements for complexity
fn analyze_block(block: &Block) -> usize {
    let mut complexity = 0;

    for stmt in &block.stmts {
        complexity += analyze_statement(stmt);
    }

    complexity
}

/// Analyzes a single statement for complexity
fn analyze_statement(stmt: &Stmt) -> usize {
    match stmt {
        Stmt::Expr(expr, _) => analyze_expression(expr),
        Stmt::Local(local) => {
            let mut complexity = 0;
            if let Some(init) = &local.init {
                complexity += analyze_expression(&init.expr);
            }
            complexity
        }
        Stmt::Item(item) => analyze_item(item),
        Stmt::Macro(_) => 0, // Macros are not analyzed for complexity
    }
}

/// Analyzes an expression for complexity
fn analyze_expression(expr: &Expr) -> usize {
    match expr {
        // Conditional expressions add complexity
        Expr::If(expr_if) => {
            let mut complexity = 1; // if condition
            complexity += analyze_expression(&expr_if.cond);
            complexity += analyze_block(&expr_if.then_branch);

            if let Some((_, else_branch)) = &expr_if.else_branch {
                complexity += analyze_expression(else_branch);
            }

            complexity
        }

        // Match expressions: base complexity + each arm
        Expr::Match(expr_match) => {
            let mut complexity = 1; // Base for match
            complexity += analyze_expression(&expr_match.expr);

            for arm in &expr_match.arms {
                complexity += analyze_match_arm(arm);
            }

            complexity
        }

        // Loop expressions add complexity
        Expr::While(expr_while) => {
            let mut complexity = 1; // while condition
            complexity += analyze_expression(&expr_while.cond);
            complexity += analyze_block(&expr_while.body);
            complexity
        }

        Expr::ForLoop(expr_for) => {
            let mut complexity = 1; // for loop
            complexity += analyze_expression(&expr_for.expr);
            complexity += analyze_block(&expr_for.body);
            complexity
        }

        Expr::Loop(expr_loop) => {
            1 + analyze_block(&expr_loop.body) // loop adds complexity
        }

        // Logical operators add complexity
        Expr::Binary(expr_binary) => {
            let mut complexity = 0;

            // && and || operators add decision points
            match expr_binary.op {
                syn::BinOp::And(_) | syn::BinOp::Or(_) => complexity += 1,
                _ => {}
            }

            complexity += analyze_expression(&expr_binary.left);
            complexity += analyze_expression(&expr_binary.right);
            complexity
        }

        // Try expressions (?) add complexity
        Expr::Try(expr_try) => 1 + analyze_expression(&expr_try.expr),

        // Return statements add complexity (except final returns)
        Expr::Return(expr_return) => {
            let mut complexity = 1; // return statement
            if let Some(expr) = &expr_return.expr {
                complexity += analyze_expression(expr);
            }
            complexity
        }

        // Break and continue add complexity
        Expr::Break(expr_break) => {
            let mut complexity = 1; // break statement
            if let Some(expr) = &expr_break.expr {
                complexity += analyze_expression(expr);
            }
            complexity
        }

        Expr::Continue(_) => 1, // continue statement

        // Block expressions
        Expr::Block(expr_block) => analyze_block(&expr_block.block),

        // Unsafe blocks
        Expr::Unsafe(expr_unsafe) => analyze_block(&expr_unsafe.block),

        // Async blocks
        Expr::Async(expr_async) => analyze_block(&expr_async.block),

        // Closures
        Expr::Closure(expr_closure) => analyze_expression(&expr_closure.body),

        // Function calls and method calls
        Expr::Call(expr_call) => {
            let mut complexity = 0;
            complexity += analyze_expression(&expr_call.func);
            for arg in &expr_call.args {
                complexity += analyze_expression(arg);
            }
            complexity
        }

        Expr::MethodCall(expr_method) => {
            let mut complexity = 0;
            complexity += analyze_expression(&expr_method.receiver);
            for arg in &expr_method.args {
                complexity += analyze_expression(arg);
            }
            complexity
        }

        // Array and tuple expressions
        Expr::Array(expr_array) => {
            let mut complexity = 0;
            for elem in &expr_array.elems {
                complexity += analyze_expression(elem);
            }
            complexity
        }

        Expr::Tuple(expr_tuple) => {
            let mut complexity = 0;
            for elem in &expr_tuple.elems {
                complexity += analyze_expression(elem);
            }
            complexity
        }

        // Field access and indexing
        Expr::Field(expr_field) => analyze_expression(&expr_field.base),
        Expr::Index(expr_index) => {
            analyze_expression(&expr_index.expr) + analyze_expression(&expr_index.index)
        }

        // Assignment expressions
        Expr::Assign(expr_assign) => {
            analyze_expression(&expr_assign.left) + analyze_expression(&expr_assign.right)
        }

        // Reference and dereference
        Expr::Reference(expr_ref) => analyze_expression(&expr_ref.expr),
        Expr::Unary(expr_unary) => analyze_expression(&expr_unary.expr),

        // Cast expressions
        Expr::Cast(expr_cast) => analyze_expression(&expr_cast.expr),

        // Range expressions
        Expr::Range(expr_range) => {
            let mut complexity = 0;
            if let Some(start) = &expr_range.start {
                complexity += analyze_expression(start);
            }
            if let Some(end) = &expr_range.end {
                complexity += analyze_expression(end);
            }
            complexity
        }

        // Struct and enum construction
        Expr::Struct(expr_struct) => {
            let mut complexity = 0;
            for field in &expr_struct.fields {
                complexity += analyze_expression(&field.expr);
            }
            if let Some(rest) = &expr_struct.rest {
                complexity += analyze_expression(rest);
            }
            complexity
        }

        // Parenthesized expressions
        Expr::Paren(expr_paren) => analyze_expression(&expr_paren.expr),

        // Group expressions
        Expr::Group(expr_group) => analyze_expression(&expr_group.expr),

        // All other expressions (literals, paths, etc.) don't add complexity
        _ => 0,
    }
}

/// Analyzes a match arm for complexity
fn analyze_match_arm(arm: &Arm) -> usize {
    let mut complexity = 1; // Each arm adds complexity

    // Analyze guard conditions
    if let Some((_, guard)) = &arm.guard {
        complexity += analyze_expression(guard);
    }

    // Analyze the arm body
    complexity += analyze_expression(&arm.body);

    complexity
}

/// Analyzes an item (nested function, etc.) for complexity
fn analyze_item(item: &Item) -> usize {
    match item {
        Item::Fn(item_fn) => calculate_cyclomatic_complexity(item_fn),
        _ => 0, // Other items don't add to the current function's complexity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_simple_function_complexity() {
        let func: ItemFn = parse_quote! {
            fn simple() {
                println!("Hello, world!");
            }
        };

        assert_eq!(calculate_cyclomatic_complexity(&func), 1);
    }

    #[test]
    fn test_if_statement_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_if(x: i32) {
                if x > 0 {
                    println!("positive");
                }
            }
        };

        assert_eq!(calculate_cyclomatic_complexity(&func), 2);
    }

    #[test]
    fn test_if_else_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_if_else(x: i32) {
                if x > 0 {
                    println!("positive");
                } else {
                    println!("not positive");
                }
            }
        };

        assert_eq!(calculate_cyclomatic_complexity(&func), 2);
    }

    #[test]
    fn test_match_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_match(x: Option<i32>) {
                match x {
                    Some(val) => println!("Got: {}", val),
                    None => println!("Got nothing"),
                }
            }
        };

        // Base 1 + match 1 + 2 arms = 4
        assert_eq!(calculate_cyclomatic_complexity(&func), 4);
    }

    #[test]
    fn test_loop_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_loops() {
                while true {
                    println!("loop");
                }

                for i in 0..10 {
                    println!("{}", i);
                }

                loop {
                    break;
                }
            }
        };

        // Base 1 + while 1 + for 1 + loop 1 + break 1 = 5
        assert_eq!(calculate_cyclomatic_complexity(&func), 5);
    }

    #[test]
    fn test_logical_operators_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_logical_ops(a: bool, b: bool, c: bool) {
                if a && b || c {
                    println!("complex condition");
                }
            }
        };

        // Base 1 + if 1 + && 1 + || 1 = 4
        assert_eq!(calculate_cyclomatic_complexity(&func), 4);
    }

    #[test]
    fn test_try_operator_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_try() -> Result<i32, &'static str> {
                let result = some_function()?;
                Ok(result)
            }
        };

        // Base 1 + try operator 1 = 2
        assert_eq!(calculate_cyclomatic_complexity(&func), 2);
    }

    #[test]
    fn test_return_statement_complexity() {
        let func: ItemFn = parse_quote! {
            fn with_early_return(x: i32) -> i32 {
                if x < 0 {
                    return 0;
                }
                x * 2
            }
        };

        // Base 1 + if 1 + return 1 = 3
        assert_eq!(calculate_cyclomatic_complexity(&func), 3);
    }

    #[test]
    fn test_nested_complexity() {
        let func: ItemFn = parse_quote! {
            fn nested_conditions(x: i32, y: i32) {
                if x > 0 {
                    if y > 0 {
                        println!("both positive");
                    } else {
                        println!("x positive, y not");
                    }
                } else {
                    println!("x not positive");
                }
            }
        };

        // Base 1 + outer if 1 + inner if 1 = 3
        assert_eq!(calculate_cyclomatic_complexity(&func), 3);
    }

    #[test]
    fn test_complex_match_with_guards() {
        let func: ItemFn = parse_quote! {
            fn complex_match(x: Option<i32>) {
                match x {
                    Some(val) if val > 0 => println!("positive: {}", val),
                    Some(val) if val < 0 => println!("negative: {}", val),
                    Some(0) => println!("zero"),
                    None => println!("none"),
                }
            }
        };

        // Base 1 + match 1 + 4 arms (each with guard or condition) = 6
        assert_eq!(calculate_cyclomatic_complexity(&func), 6);
    }
}
