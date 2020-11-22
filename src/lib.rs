mod lexer;
mod parser;
mod readline;
mod run_test;


pub use lexer::*;
pub use parser::*;
pub use readline::readline;
pub use run_test::run_test;

fn eval_const(n: &Node) -> Node {
    let mut ret_node = Node::new();
    if let Token::Ident(ident) = &n.op {
        match ident.as_str() {
            "pi" => {
                ret_node.ty = NodeType::FNum;
                ret_node.fvalue = std::f64::consts::PI;
                ret_node
            }
            _ => Node::new(),
        }
    } else {
        Node::new()
    }
}

fn eval_func(n: &Node) -> Node {
    let mut ret_node = Node::new();
    if let Token::Ident(ident) = &n.op {
        match ident.as_str() {
            "sin" => {
                ret_node.ty = NodeType::FNum;
                let mut arg = eval(&n.child[0]);
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

fn eval_binop(n: &Node) -> Node {
    // println!("eval_binop {:?}", n);
    assert!(n.child.len() == 2);
    let lhs = eval(&n.child[0]);
    let rhs = eval(&n.child[1]);
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

pub fn eval(n: &Node) -> Node {
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
                    let n = eval_binop(&n.child[0]);
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
                    let n = eval_binop(&n.child[0]);
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
        NodeType::BinOp => eval_binop(n),
        NodeType::Var => eval_const(n),
        NodeType::Func => eval_func(n),
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

    #[test]
    fn test_eval() {
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1+2".to_string())).unwrap()))),
            "Num(3)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1+2*3".to_string())).unwrap()))),
            "Num(7)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1*2+3".to_string())).unwrap()))),
            "Num(5)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1+2+3".to_string())).unwrap()))),
            "Num(6)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("(1+2)*3".to_string())).unwrap()))
            ),
            "Num(9)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("-2".to_string())).unwrap()))),
            "Num(-2)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(
                    &(lexer("-9223372036854775807".to_string())).unwrap()
                ))
            ),
            "Num(-9223372036854775807)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("1.1+2.2".to_string())).unwrap()))
            ),
            "FNum(3.3000000000000003)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("-(2+3)".to_string())).unwrap()))
            ),
            "Num(-5)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("+(2+3)".to_string())).unwrap()))
            ),
            "Num(5)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("+(2+3)".to_string())).unwrap()))
            ),
            "Num(5)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1+2".to_string())).unwrap()))),
            "Num(3)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1.0+2".to_string())).unwrap()))),
            "FNum(3)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("1+2.0".to_string())).unwrap()))),
            "FNum(3)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("1.0+2.0".to_string())).unwrap()))
            ),
            "FNum(3)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("(1+2.0)*3".to_string())).unwrap()))
            ),
            "FNum(9)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("pi".to_string())).unwrap()))),
            "FNum(3.141592653589793)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&(lexer("2k*3u".to_string())).unwrap()))),
            "FNum(0.006)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("sin(0.0)".to_string())).unwrap()))
            ),
            "FNum(0)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&(lexer("sin(0)".to_string())).unwrap()))
            ),
            "FNum(0)"
        );
        let n = eval(&parse(&(lexer("sin(pi)".to_string())).unwrap()));
        assert!(n.ty == NodeType::FNum);
        assert!((n.fvalue.abs()) < 1e-10);
        let n = eval(&parse(&(lexer("sin(pi/2)".to_string())).unwrap()));
        assert!(n.ty == NodeType::FNum);
        assert!(((n.fvalue - 1.0).abs()) < 1e-10);
    }
}
