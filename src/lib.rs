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

fn eval_const(env: &mut Env, n: &Node) -> Node {
    let mut ret_node = Node::new();
    if let Token::Ident(ident) = &n.op {
        if let Some(constant) = env.is_const(ident.as_str()) {
            ret_node.ty = NodeType::FNum;
            ret_node.fvalue = constant;
            return ret_node;
        }
    }
    Node::new()
}

fn eval_func(env: &mut Env, n: &Node) -> Node {
    let mut ret_node = Node::new();
    if let Token::Ident(ident) = &n.op {
        match ident.as_str() {
            "sin" => {
                ret_node.ty = NodeType::FNum;
                let mut arg = eval(env, &n.child[0]);
                if arg.ty == NodeType::Num {
                    arg.ty = NodeType::FNum;
                    arg.fvalue = arg.value as f64;
                }
                ret_node.fvalue = arg.fvalue.sin();
                ret_node
            }
            _ => Node::new(),
        }
    } else {
        Node::new()
    }
}

fn eval_binop(env: &mut Env, n: &Node) -> Node {
    // println!("eval_binop {:?}", n);
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
            ret_node.fvalue = if lhs.ty == NodeType::Num {
                lhs.value as f64
            } else {
                lhs.fvalue
            } + if rhs.ty == NodeType::Num {
                rhs.value as f64
            } else {
                rhs.fvalue
            };
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
            ret_node.fvalue = if lhs.ty == NodeType::Num {
                lhs.value as f64
            } else {
                lhs.fvalue
            } - if rhs.ty == NodeType::Num {
                rhs.value as f64
            } else {
                rhs.fvalue
            };
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
            ret_node.fvalue = if lhs.ty == NodeType::Num {
                lhs.value as f64
            } else {
                lhs.fvalue
            } * if rhs.ty == NodeType::Num {
                rhs.value as f64
            } else {
                rhs.fvalue
            };
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
            ret_node.fvalue = if lhs.ty == NodeType::Num {
                lhs.value as f64
            } else {
                lhs.fvalue
            } / if rhs.ty == NodeType::Num {
                rhs.value as f64
            } else {
                rhs.fvalue
            };
            return ret_node;
        }
    }
    Node::new()
}

pub fn eval(env: &mut Env, n: &Node) -> Node {
    // println!("eval {:?}", n);
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
                    let n = eval_binop(env, &n.child[0]);
                    if n.ty == NodeType::FNum {
                        let mut ret_node = Node::new();
                        ret_node.ty = NodeType::FNum;
                        ret_node.fvalue = n.fvalue;
                        return ret_node;
                    }
                    if n.ty == NodeType::Num {
                        let mut ret_node = Node::new();
                        ret_node.ty = NodeType::Num;
                        ret_node.value = n.value;
                        return ret_node;
                    }
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
        assert_eq!(
            eval_as_string(&mut env, "1.1+2.2"),
            "FNum(3.3000000000000003)".to_string()
        );
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

        let n = parse(&mut env, &(lexer("sin(pi)".to_string())).unwrap());
        let n = eval(&mut env, &n);
        assert!(n.ty == NodeType::FNum);
        assert!((n.fvalue.abs()) < 1e-10);

        let n = parse(&mut env, &(lexer("sin(pi/2)".to_string())).unwrap());
        let n = eval(&mut env, &n);
        assert!(n.ty == NodeType::FNum);
        assert!(((n.fvalue - 1.0).abs()) < 1e-10);
    }
}
