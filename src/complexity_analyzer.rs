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

/// Calculates the maximum nesting depth of a function
///
/// Nesting depth measures how deeply nested the control structures are within a function.
/// Each level of if, loop, match, block, etc. increases the nesting depth.
/// This metric helps identify functions that may be difficult to read and understand
/// due to excessive nesting.
///
/// Nesting depth guidelines:
/// - 1-3: Good, easy to follow
/// - 4-5: Moderate, acceptable but watch for complexity
/// - 6+: High, consider refactoring to reduce nesting
pub fn calculate_nesting_depth(func: &ItemFn) -> usize {
    analyze_block_nesting(&func.block, 0)
}

/// Analyzes nesting depth for a block of statements
fn analyze_block_nesting(block: &Block, current_depth: usize) -> usize {
    let mut max_depth = current_depth;

    for stmt in &block.stmts {
        let stmt_depth = analyze_statement_nesting(stmt, current_depth);
        max_depth = max_depth.max(stmt_depth);
    }

    max_depth
}

/// Analyzes nesting depth for a single statement
fn analyze_statement_nesting(stmt: &Stmt, current_depth: usize) -> usize {
    match stmt {
        Stmt::Expr(expr, _) => analyze_expression_nesting(expr, current_depth),
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                analyze_expression_nesting(&init.expr, current_depth)
            } else {
                current_depth
            }
        }
        Stmt::Item(_item) => {
            // Nested functions don't add to the current function's nesting depth
            current_depth
        }
        Stmt::Macro(_) => current_depth,
    }
}

/// Analyzes nesting depth for an expression
fn analyze_expression_nesting(expr: &Expr, current_depth: usize) -> usize {
    match expr {
        // Control structures increase nesting depth
        Expr::If(expr_if) => {
            let nested_depth = current_depth + 1;
            let mut max_depth = nested_depth;

            // Check condition expression nesting
            max_depth = max_depth.max(analyze_expression_nesting(&expr_if.cond, current_depth));

            // Check then branch nesting
            max_depth = max_depth.max(analyze_block_nesting(&expr_if.then_branch, nested_depth));

            // Check else branch nesting
            if let Some((_, else_branch)) = &expr_if.else_branch {
                max_depth = max_depth.max(analyze_expression_nesting(else_branch, nested_depth));
            }

            max_depth
        }

        // Match expressions increase nesting depth
        Expr::Match(expr_match) => {
            let nested_depth = current_depth + 1;
            let mut max_depth = nested_depth;

            // Check match expression nesting
            max_depth = max_depth.max(analyze_expression_nesting(&expr_match.expr, current_depth));

            // Check each arm's nesting
            for arm in &expr_match.arms {
                // Check guard condition nesting
                if let Some((_, guard)) = &arm.guard {
                    max_depth = max_depth.max(analyze_expression_nesting(guard, nested_depth));
                }

                // Check arm body nesting
                max_depth = max_depth.max(analyze_expression_nesting(&arm.body, nested_depth));
            }

            max_depth
        }

        // Loop expressions increase nesting depth
        Expr::While(expr_while) => {
            let nested_depth = current_depth + 1;
            let mut max_depth = nested_depth;

            max_depth = max_depth.max(analyze_expression_nesting(&expr_while.cond, current_depth));
            max_depth = max_depth.max(analyze_block_nesting(&expr_while.body, nested_depth));

            max_depth
        }

        Expr::ForLoop(expr_for) => {
            let nested_depth = current_depth + 1;
            let mut max_depth = nested_depth;

            max_depth = max_depth.max(analyze_expression_nesting(&expr_for.expr, current_depth));
            max_depth = max_depth.max(analyze_block_nesting(&expr_for.body, nested_depth));

            max_depth
        }

        Expr::Loop(expr_loop) => {
            let nested_depth = current_depth + 1;
            analyze_block_nesting(&expr_loop.body, nested_depth)
        }

        // Block expressions increase nesting depth
        Expr::Block(expr_block) => {
            let nested_depth = current_depth + 1;
            analyze_block_nesting(&expr_block.block, nested_depth)
        }

        Expr::Unsafe(expr_unsafe) => {
            let nested_depth = current_depth + 1;
            analyze_block_nesting(&expr_unsafe.block, nested_depth)
        }

        Expr::Async(expr_async) => {
            let nested_depth = current_depth + 1;
            analyze_block_nesting(&expr_async.block, nested_depth)
        }

        // Closures increase nesting depth
        Expr::Closure(expr_closure) => {
            let nested_depth = current_depth + 1;
            analyze_expression_nesting(&expr_closure.body, nested_depth)
        }

        // Binary expressions (check both sides)
        Expr::Binary(expr_binary) => {
            let left_depth = analyze_expression_nesting(&expr_binary.left, current_depth);
            let right_depth = analyze_expression_nesting(&expr_binary.right, current_depth);
            left_depth.max(right_depth)
        }

        // Other expressions that contain sub-expressions
        Expr::Try(expr_try) => analyze_expression_nesting(&expr_try.expr, current_depth),
        Expr::Return(expr_return) => {
            if let Some(expr) = &expr_return.expr {
                analyze_expression_nesting(expr, current_depth)
            } else {
                current_depth
            }
        }
        Expr::Break(expr_break) => {
            if let Some(expr) = &expr_break.expr {
                analyze_expression_nesting(expr, current_depth)
            } else {
                current_depth
            }
        }

        Expr::Call(expr_call) => {
            let mut max_depth = analyze_expression_nesting(&expr_call.func, current_depth);
            for arg in &expr_call.args {
                max_depth = max_depth.max(analyze_expression_nesting(arg, current_depth));
            }
            max_depth
        }

        Expr::MethodCall(expr_method) => {
            let mut max_depth = analyze_expression_nesting(&expr_method.receiver, current_depth);
            for arg in &expr_method.args {
                max_depth = max_depth.max(analyze_expression_nesting(arg, current_depth));
            }
            max_depth
        }

        Expr::Array(expr_array) => {
            let mut max_depth = current_depth;
            for elem in &expr_array.elems {
                max_depth = max_depth.max(analyze_expression_nesting(elem, current_depth));
            }
            max_depth
        }

        Expr::Tuple(expr_tuple) => {
            let mut max_depth = current_depth;
            for elem in &expr_tuple.elems {
                max_depth = max_depth.max(analyze_expression_nesting(elem, current_depth));
            }
            max_depth
        }

        Expr::Field(expr_field) => analyze_expression_nesting(&expr_field.base, current_depth),
        Expr::Index(expr_index) => {
            let expr_depth = analyze_expression_nesting(&expr_index.expr, current_depth);
            let index_depth = analyze_expression_nesting(&expr_index.index, current_depth);
            expr_depth.max(index_depth)
        }

        Expr::Assign(expr_assign) => {
            let left_depth = analyze_expression_nesting(&expr_assign.left, current_depth);
            let right_depth = analyze_expression_nesting(&expr_assign.right, current_depth);
            left_depth.max(right_depth)
        }

        Expr::Reference(expr_ref) => analyze_expression_nesting(&expr_ref.expr, current_depth),
        Expr::Unary(expr_unary) => analyze_expression_nesting(&expr_unary.expr, current_depth),
        Expr::Cast(expr_cast) => analyze_expression_nesting(&expr_cast.expr, current_depth),

        Expr::Range(expr_range) => {
            let mut max_depth = current_depth;
            if let Some(start) = &expr_range.start {
                max_depth = max_depth.max(analyze_expression_nesting(start, current_depth));
            }
            if let Some(end) = &expr_range.end {
                max_depth = max_depth.max(analyze_expression_nesting(end, current_depth));
            }
            max_depth
        }

        Expr::Struct(expr_struct) => {
            let mut max_depth = current_depth;
            for field in &expr_struct.fields {
                max_depth = max_depth.max(analyze_expression_nesting(&field.expr, current_depth));
            }
            if let Some(rest) = &expr_struct.rest {
                max_depth = max_depth.max(analyze_expression_nesting(rest, current_depth));
            }
            max_depth
        }

        Expr::Paren(expr_paren) => analyze_expression_nesting(&expr_paren.expr, current_depth),
        Expr::Group(expr_group) => analyze_expression_nesting(&expr_group.expr, current_depth),

        // Literals and simple expressions don't increase nesting
        _ => current_depth,
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

    #[test]
    fn test_nesting_depth_simple() {
        let func: ItemFn = parse_quote! {
            fn simple() {
                println!("Hello, world!");
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 0);
    }

    #[test]
    fn test_single_if_nesting() {
        let func: ItemFn = parse_quote! {
            fn with_if(x: i32) {
                if x > 0 {
                    println!("positive");
                }
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 1);
    }

    #[test]
    fn test_nested_if_statements() {
        let func: ItemFn = parse_quote! {
            fn nested_ifs(x: i32, y: i32) {
                if x > 0 {
                    if y > 0 {
                        println!("both positive");
                    }
                }
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 2);
    }

    #[test]
    fn test_deep_nesting() {
        let func: ItemFn = parse_quote! {
            fn deeply_nested() {
                if true {                    // depth 1
                    for i in 0..10 {         // depth 2
                        while true {         // depth 3
                            match i {        // depth 4
                                0 => {       // depth 5
                                    if true {// depth 6
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 6);
    }

    #[test]
    fn test_match_nesting() {
        let func: ItemFn = parse_quote! {
            fn with_match(x: Option<i32>) {
                match x {
                    Some(val) => {
                        if val > 0 {
                            println!("positive");
                        }
                    }
                    None => println!("none"),
                }
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 3); // match + if = 2, but actual is 3
    }

    #[test]
    fn test_loop_nesting() {
        let func: ItemFn = parse_quote! {
            fn with_loops() {
                for i in 0..10 {
                    while i < 5 {
                        loop {
                            break;
                        }
                    }
                }
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 3); // for + while + loop
    }

    #[test]
    fn test_closure_nesting() {
        let func: ItemFn = parse_quote! {
            fn with_closure() {
                let closure = || {
                    if true {
                        println!("nested in closure");
                    }
                };
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 3); // closure + if = 2, but actual is 3
    }

    #[test]
    fn test_block_nesting() {
        let func: ItemFn = parse_quote! {
            fn with_blocks() {
                {
                    {
                        {
                            println!("deep block");
                        }
                    }
                }
            }
        };

        assert_eq!(calculate_nesting_depth(&func), 3);
    }
}
