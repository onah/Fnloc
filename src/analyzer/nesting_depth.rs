use syn::{Block, Expr, ItemFn, Stmt};

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
