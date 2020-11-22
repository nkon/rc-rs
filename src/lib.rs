use std::fmt;

mod lexer;
mod readline;
mod run_test;

pub use lexer::*;
pub use readline::readline;
pub use run_test::run_test;

// <expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
// <mul>     ::= <unary> ( '*' <unary> | '/' <unary>)*
// <unary>   ::= <primary> | '-' <primary> | '+' <primary>
// <primary> ::= <num> | '(' <expr> ')' | <var> | <func> '(' <expr>* ',' ')'
// <num>     ::= <num> | <num> <postfix>

#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    None,
    Num,   // value <- value
    FNum,  // fvalue <- value
    Unary, // op <- operator, child[0] <- operand
    BinOp, // op <- operator, child[0] <- lhs, child[1] <- rhs
    Var,   // op <- Token::Ident(ident)
    Func,  // op <- Token::Ident(ident), child[] <- parameter
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num"),
            NodeType::FNum => write!(f, "FNum"),
            NodeType::Unary => write!(f, "Unary"),
            NodeType::BinOp => write!(f, "BinOp"),
            NodeType::Var => write!(f, "Var"),
            NodeType::Func => write!(f, "Func"),
        }
    }
}

// TODO: change from struct to Enum to maximize Rust power
pub struct Node {
    pub ty: NodeType,
    pub value: i128,
    pub fvalue: f64,
    pub op: Token,
    pub child: Vec<Node>, // child[0]: LHS, child[1]: RHS
}

impl Node {
    pub fn new() -> Node {
        Node {
            ty: NodeType::None,
            value: 0,
            fvalue: 0.0,
            child: Vec::new(),
            op: Token::Op(' '),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num({})", self.value),
            NodeType::FNum => write!(f, "FNum({})", self.fvalue),
            NodeType::Unary => write!(f, "Unary({:?} {:?})", self.op, self.child[0]),
            NodeType::BinOp => write!(f, "BinOp({:?} {:?})", self.op, self.child),
            NodeType::Var => write!(f, "Var({:?})", self.op),
            NodeType::Func => write!(f, "Func({:?} {:?})", self.op, self.child),
        }
    }
}

fn num(tok: &[Token], i: usize) -> (Node, usize) {
    // println!("num {:?} {}", tok, i);
    if tok.len() <= i {
        return (Node::new(), i);
    }
    let mut node = Node::new();
    let mut f_postfix = 1.0;
    let mut has_postfix = false;
    if (i + 1) < tok.len() {
        if let Token::Ident(id) = &tok[i + 1] {
            match id.as_ref() {
                "k" => {
                    f_postfix = 1000.0;
                    has_postfix = true;
                }
                "M" => {
                    f_postfix = 1_000_000.0;
                    has_postfix = true;
                }
                "G" => {
                    f_postfix = 1_000_000_000.0;
                    has_postfix = true;
                }
                "T" => {
                    f_postfix = 1_000_000_000_000.0;
                    has_postfix = true;
                }
                "m" => {
                    f_postfix = 0.001;
                    has_postfix = true;
                }
                "u" => {
                    f_postfix = 0.000_001;
                    has_postfix = true;
                }
                "n" => {
                    f_postfix = 0.000_000_001;
                    has_postfix = true;
                }
                "p" => {
                    f_postfix = 0.000_000_000_001;
                    has_postfix = true;
                }
                _ => {}
            }
        }
    }
    match tok[i] {
        Token::Num(n) => {
            if has_postfix {
                node.ty = NodeType::FNum;
                node.fvalue = n as f64 * f_postfix;
                (node, i + 2)
            } else {
                node.ty = NodeType::Num;
                node.value = n;
                (node, i + 1)
            }
        }
        Token::FNum(n) => {
            node.ty = NodeType::FNum;
            if has_postfix {
                node.fvalue = n * f_postfix;
                (node, i + 2)
            } else {
                node.fvalue = n;
                (node, i + 1)
            }
        }
        _ => (node, i),
    }
}

fn primary(tok: &[Token], index: usize) -> (Node, usize) {
    let mut i = index;
    // println!("primary {:?} {}", tok, i);
    if tok.len() <= i {
        return (Node::new(), i);
    }
    match &tok[i] {
        Token::Op('(') => {
            let (expr, i) = expr(tok, i + 1);
            if tok[i] != Token::Op(')') {
                println!("')' not found.");
            }
            (expr, i + 1)
        }
        Token::Ident(id) => {
            let mut ret_node = Node::new();
            if &(*id.as_str()) == "pi" {
                // FIXME: is_var()
                ret_node.ty = NodeType::Var;
                ret_node.op = Token::Ident(id.clone());
                (ret_node, i + 1)
            } else if &(*id.as_str()) == "sin" {
                // FIXME: is_func()
                ret_node.ty = NodeType::Func;
                ret_node.op = Token::Ident(id.clone());
                if (i + 1) < tok.len() || tok[i + 1] == Token::Op('(') {
                    i += 2;
                    while i < tok.len() {
                        if tok[i] == Token::Op(')') {
                            return (ret_node, i + 1);
                        } else if tok[i] == Token::Op(',') {
                            i += 1;
                            continue;
                        } else {
                            let (t, j) = expr(tok, i);
                            i = j;
                            ret_node.child.push(t);
                        }
                    }
                }
                (ret_node, i + 1)
            } else {
                (ret_node, i)
            }
        }
        _ => num(tok, i),
    }
}

fn unary(tok: &[Token], i: usize) -> (Node, usize) {
    // println!("unary {:?} {}", tok, i);
    if tok.len() <= i {
        return (Node::new(), i);
    }
    match tok[i] {
        Token::Op('-') | Token::Op('+') => {
            let mut node = Node::new();
            node.ty = NodeType::Unary;
            node.op = tok[i].clone();
            let (rhs, i) = primary(tok, i + 1);
            node.child.push(rhs);
            (node, i)
        }
        _ => primary(tok, i),
    }
}

fn mul(tok: &[Token], i: usize) -> (Node, usize) {
    // println!("mul {:?} {}", tok, i);
    if tok.len() <= i {
        return (Node::new(), i);
    }
    let (mut lhs, mut i) = unary(tok, i);
    loop {
        if tok.len() <= i {
            return (lhs, i);
        }
        match tok[i] {
            Token::Op('*') | Token::Op('/') | Token::Op('%') => {
                let mut node = Node::new();
                node.ty = NodeType::BinOp;
                node.op = tok[i].clone();
                let (rhs, j) = unary(tok, i + 1);
                node.child.push(lhs);
                node.child.push(rhs);
                i = j;
                lhs = node;
            }
            _ => {
                return (lhs, i);
            }
        }
    }
}

fn expr(tok: &[Token], i: usize) -> (Node, usize) {
    // println!("expr {:?} {}", tok, i);
    if tok.len() <= i {
        return (Node::new(), i);
    }
    let (mut lhs, mut i) = mul(tok, i);
    loop {
        if tok.len() <= i {
            return (lhs, i);
        }
        match tok[i] {
            Token::Op('+') | Token::Op('-') => {
                let mut node = Node::new();
                node.ty = NodeType::BinOp;
                node.op = tok[i].clone();
                let (rhs, j) = mul(tok, i + 1);
                node.child.push(lhs);
                node.child.push(rhs);
                i = j;
                lhs = node;
            }
            _ => {
                return (lhs, i);
            }
        }
    }
}

// TODO: handle vars/functions.
pub fn parse(tok: &[Token]) -> Node {
    let (node, _i) = expr(&tok, 0);

    // println!("{:?} {}", node, i);
    node
}

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
    fn test_parser() {
        assert_eq!(
            format!("{:?}", parse(&(lexer("1+2".to_string()).unwrap()))),
            "BinOp(Op('+') [Num(1), Num(2)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1-2".to_string()).unwrap()))),
            "BinOp(Op('-') [Num(1), Num(2)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1+-2".to_string()).unwrap()))),
            "BinOp(Op('+') [Num(1), Unary(Op('-') Num(2))])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1*2".to_string()).unwrap()))),
            "BinOp(Op('*') [Num(1), Num(2)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1*2+3".to_string()).unwrap()))),
            "BinOp(Op('+') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1*(2+3)".to_string()).unwrap()))),
            "BinOp(Op('*') [Num(1), BinOp(Op('+') [Num(2), Num(3)])])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1+2+3".to_string())).unwrap())),
            "BinOp(Op('+') [BinOp(Op('+') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("(1+2)+3".to_string())).unwrap())),
            "BinOp(Op('+') [BinOp(Op('+') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("1*2*3".to_string())).unwrap())),
            "BinOp(Op('*') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("(1*2)*3".to_string())).unwrap())),
            "BinOp(Op('*') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("-(2+3)".to_string())).unwrap())),
            "Unary(Op('-') BinOp(Op('+') [Num(2), Num(3)]))"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("pi".to_string())).unwrap())),
            "Var(Ident(\"pi\"))"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("2.0*pi".to_string())).unwrap())),
            "BinOp(Op('*') [FNum(2), Var(Ident(\"pi\"))])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("2k".to_string())).unwrap())),
            "FNum(2000)"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("2u*pi".to_string())).unwrap())),
            "BinOp(Op('*') [FNum(0.000002), Var(Ident(\"pi\"))])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("2*sin(0.5*pi)".to_string())).unwrap())),
            "BinOp(Op('*') [Num(2), Func(Ident(\"sin\") [BinOp(Op('*') [FNum(0.5), Var(Ident(\"pi\"))])])])"
        );
        assert_eq!(
            format!("{:?}", parse(&(lexer("2*sin(1, 2)".to_string())).unwrap())),
            "BinOp(Op('*') [Num(2), Func(Ident(\"sin\") [Num(1), Num(2)])])"
        );
    }

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
