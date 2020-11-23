mod env;
mod lexer;
mod parser;
mod readline;
mod run_test;

pub use env::*;
pub use lexer::*;
pub use parser::*;
pub use readline::readline;
pub use run_test::run_test;

pub fn eval_fvalue(_env: &mut Env, n: &Node) -> f64 {
    match n.ty {
        NodeType::Num => n.value as f64,
        NodeType::FNum => n.fvalue,
        // NodeType::None => unreachable!(),
        // BUG: commentout -> come here
        _ => {
            // let node = eval(env, n);
            // eval_fvalue(env, &node)
            // BUG: commentout -> come here
            0.0
        }
    }
}

fn eval_const(env: &mut Env, n: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_const {:?}\r", n);
    }
    let mut ret_node = Node::new();
    if let Token::Ident(ident) = &n.op {
        if let Some(constant) = env.is_const(ident.as_str()) {
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = constant;
            return ret_node;
        }
    }
    eprintln!("Error: unknown constant: {:?}\r", n);
    Node::new()
}

fn eval_func(env: &mut Env, n: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_func {:?}\r", n);
    }
    let mut ret_node = Node::new();
    if let Token::Ident(ident) = &n.op {
        if let Some(func_tupple) = env.is_func(ident.as_str()) {
            let mut params: Vec<Node> = Vec::new();
            for i in 0..n.child.len() {
                let param = eval(env, &n.child[i]);
                let mut n_param = Node::new();
                match param.ty {
                    NodeType::Num => {
                        n_param.ty = NodeType::FNum;
                        n_param.fvalue = param.value as f64;
                    }
                    NodeType::FNum => {
                        n_param.ty = param.ty;
                        n_param.fvalue = param.fvalue;
                    }
                    _ => {}
                }
                params.push(n_param);
            }
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = func_tupple.0(env, &params);
            return ret_node;
        }
    }
    eprintln!("Error: unknown function: {:?}\r", n);
    Node::new()
}

fn eval_binop(env: &mut Env, n: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_binop {:?}\r", n);
    }
    assert!(n.child.len() == 2);
    let lhs = eval(env, &n.child[0]);
    let rhs = eval(env, &n.child[1]);
    let mut ret_node = Node::new();
    if n.op == Token::Op('+') {
        if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
            ret_node.ty = NodeType::Num;
            ret_node.value = lhs.value + rhs.value;
            return ret_node;
        } else {
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = eval_fvalue(env, &lhs) + eval_fvalue(env, &rhs);
            return ret_node;
        }
    }
    if n.op == Token::Op('-') {
        if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
            ret_node.ty = NodeType::Num;
            ret_node.value = lhs.value - rhs.value;
            return ret_node;
        } else {
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = eval_fvalue(env, &lhs) - eval_fvalue(env, &rhs);
            return ret_node;
        }
    }
    if n.op == Token::Op('*') {
        if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
            ret_node.ty = NodeType::Num;
            ret_node.value = lhs.value * rhs.value;
            return ret_node;
        } else {
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = eval_fvalue(env, &lhs) * eval_fvalue(env, &rhs);
            return ret_node;
        }
    }
    if n.op == Token::Op('/') {
        if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
            ret_node.ty = NodeType::Num;
            ret_node.value = lhs.value / rhs.value;
            return ret_node;
        } else {
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = eval_fvalue(env, &lhs) / eval_fvalue(env, &rhs);
            return ret_node;
        }
    }
    if n.op == Token::Op('%') {
        if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
            ret_node.ty = NodeType::Num;
            ret_node.value = lhs.value % rhs.value;
            return ret_node;
        } else {
            return ret_node;
        }
    }
    eprintln!("Error: binary operator: {:?}\r", n);
    Node::new()
}

pub fn eval(env: &mut Env, n: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval {:?}\r", n);
    }
    match n.ty {
        NodeType::Num => {
            let mut ret_node = Node::new();
            ret_node.ty = NodeType::Num;
            ret_node.value = n.value;
            ret_node
        }
        NodeType::FNum => {
            let mut ret_node = Node::new();
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = n.fvalue;
            ret_node
        }
        NodeType::Unary => {
            if n.op == Token::Op('-') {
                let mut ret_node = Node::new();
                if n.child[0].ty == NodeType::Num {
                    ret_node.ty = NodeType::Num;
                    ret_node.value = -n.child[0].value;
                    return ret_node;
                }
                if n.child[0].ty == NodeType::FNum {
                    ret_node.ty = NodeType::FNum;
                    ret_node.fvalue = -n.child[0].fvalue;
                    return ret_node;
                }
                if n.child[0].ty == NodeType::BinOp {
                    let n = eval_binop(env, &n.child[0]);
                    if n.ty == NodeType::FNum {
                        let mut ret_node = Node::new();
                        ret_node.ty = NodeType::FNum;
                        ret_node.fvalue = -n.fvalue;
                        return ret_node;
                    } else if n.ty == NodeType::Num {
                        let mut ret_node = Node::new();
                        ret_node.ty = NodeType::Num;
                        ret_node.value = -n.value;
                        return ret_node;
                    }
                }
            } else if n.op == Token::Op('+') {
                let mut ret_node = Node::new();
                if n.child[0].ty == NodeType::Num {
                    ret_node.ty = NodeType::Num;
                    ret_node.value = n.child[0].value;
                    return ret_node;
                }
                if n.child[0].ty == NodeType::FNum {
                    ret_node.ty = NodeType::FNum;
                    ret_node.fvalue = n.child[0].fvalue;
                    return ret_node;
                }
                if n.child[0].ty == NodeType::BinOp {
                    return eval_binop(env, &n.child[0]);
                }
            }
            let mut ret_node = Node::new();
            ret_node.ty = n.ty;
            ret_node.value = n.value;
            ret_node.fvalue = n.fvalue;
            ret_node
        }
        NodeType::BinOp => eval_binop(env, n),
        NodeType::Var => eval_const(env, n),
        NodeType::Func => eval_func(env, n),
        _ => {
            let mut ret_node = Node::new();
            ret_node.ty = n.ty;
            ret_node.value = n.value;
            ret_node.fvalue = n.fvalue;
            ret_node
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_as_string(env: &mut Env, input: &str) -> String {
        let n = parse(env, &(lexer(input.to_string())).unwrap());
        let n = eval(env, &n);
        format!("{:?}", n)
    }

    fn eval_as_f64(env: &mut Env, input: &str) -> f64 {
        let n = parse(env, &(lexer(input.to_string())).unwrap());
        let n = eval(env, &n);
        assert!(n.ty == NodeType::FNum);
        n.fvalue
    }

    #[test]
    fn test_eval() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(eval_as_string(&mut env, "1+2"), "Num(3)".to_string());
        assert_eq!(eval_as_string(&mut env, "1+2*3"), "Num(7)".to_string());
        assert_eq!(eval_as_string(&mut env, "1*2+3"), "Num(5)".to_string());
        assert_eq!(eval_as_string(&mut env, "1+2+3"), "Num(6)".to_string());
        assert_eq!(eval_as_string(&mut env, "(1+2)*3"), "Num(9)".to_string());
        assert_eq!(eval_as_string(&mut env, "-2"), "Num(-2)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "-9223372036854775807"),
            "Num(-9223372036854775807)".to_string()
        );
        assert!(((eval_as_f64(&mut env, "1.1+2.2") - 3.3).abs()) < 1e-10);
        assert_eq!(eval_as_string(&mut env, "-(2+3)"), "Num(-5)".to_string());
        assert_eq!(eval_as_string(&mut env, "+(2+3)"), "Num(5)".to_string());
        assert_eq!(eval_as_string(&mut env, "1.0+2"), "FNum(3)".to_string());
        assert_eq!(eval_as_string(&mut env, "1+2.0"), "FNum(3)".to_string());
        assert_eq!(eval_as_string(&mut env, "(1+2.0)*3"), "FNum(9)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "pi"),
            "FNum(3.141592653589793)".to_string()
        );
        assert_eq!(eval_as_string(&mut env, "2k*3u"), "FNum(0.006)".to_string());
        assert_eq!(eval_as_string(&mut env, "sin(0.0)"), "FNum(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "sin(0)"), "FNum(0)".to_string());
        assert!((eval_as_f64(&mut env, "sin(pi)").abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "sin(pi/2)") - 1.0).abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "abs(-2)") - 2.0).abs()) < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "max2(1.0, 2.0)"),
            "FNum(2)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "max2(1, 2)"),
            "FNum(2)".to_string()
        );
        assert_eq!(eval_as_string(&mut env, "sin(0))"), "FNum(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "1%3"), "Num(1)".to_string());
        assert_eq!(eval_as_string(&mut env, "2%3"), "Num(2)".to_string());
        assert_eq!(eval_as_string(&mut env, "3%3"), "Num(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "3.0%3"), "None".to_string());
        assert_eq!(eval_as_string(&mut env, "1/3"), "Num(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "3/3"), "Num(1)".to_string());
        assert_eq!(eval_as_string(&mut env, "3.0/2"), "FNum(1.5)".to_string());
    }
}
