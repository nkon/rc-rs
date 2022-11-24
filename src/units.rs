use super::*;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn eval_units_mul(env: &mut Env, lhs_u: &Node, rhs_u: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_mul {:?} {:?}\r", lhs_u, rhs_u);
    }
    match (lhs_u, rhs_u) {
        (Node::Units(lhs_i_p), _) => {
            // unpack Units() and call recursive
            if let Node::Units(rhs_i_p) = rhs_u {
                eval_units_mul(env, lhs_i_p, rhs_i_p)
            } else {
                eval_units_mul(env, lhs_i_p, rhs_u)
            }
        }
        (_, Node::Units(rhs_i_p)) => {
            // unpack Units() and call recursive
            eval_units_mul(env, lhs_u, rhs_i_p)
        }
        (Node::None, Node::None) => Node::Units(Box::new(Node::None)),
        (Node::None, _) => {
            // (lhs_u == None) ==> return rhs_u
            rhs_u.clone()
        }
        (_, Node::None) => {
            // (rhs_u == None) ==> return lhs_u
            lhs_u.clone()
        }
        (_, _) => Node::Units(Box::new(Node::BinOp(
            Token::Op(TokenOp::Mul),
            Box::new(lhs_u.clone()),
            Box::new(rhs_u.clone()),
        ))),
    }
}

pub fn eval_units_div(env: &mut Env, lhs_u: &Node, rhs_u: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_div {:?} {:?}\r", lhs_u, rhs_u);
    }
    match (lhs_u, rhs_u) {
        (Node::Units(lhs_i_p), _) => {
            // unpack Units() and call recursive
            if let Node::Units(rhs_i_p) = rhs_u {
                eval_units_div(env, lhs_i_p, rhs_i_p)
            } else {
                eval_units_div(env, lhs_i_p, rhs_u)
            }
        }
        (_, Node::Units(rhs_i_p)) => {
            // unpack Units() and call recursive
            eval_units_div(env, lhs_u, rhs_i_p)
        }
        (Node::None, Node::None) => Node::Units(Box::new(Node::None)),
        (Node::None, _) => {
            // (lhs_u == None) ==> return (1/rhs_u)
            Node::Units(Box::new(Node::BinOp(
                Token::Op(TokenOp::Div),
                Box::new(Node::Num(1, Box::new(Node::Units(Box::new(Node::None))))),
                Box::new(rhs_u.clone()),
            )))
        }
        (_, _) => Node::Units(Box::new(Node::BinOp(
            Token::Op(TokenOp::Div),
            Box::new(lhs_u.clone()),
            Box::new(rhs_u.clone()),
        ))),
    }
}

fn units_reduce_impl(env: &mut Env, units: Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_reduce_impl {:?}\r", units);
    }
    match units.clone() {
        Node::BinOp(op, lhs, rhs) => match op {
            Token::Op(TokenOp::Mul) => match (*lhs.clone(), *rhs.clone()) {
                // (g/m)*s => (g*s)/m
                (Node::BinOp(Token::Op(TokenOp::Div), llhs, lrhs), _) => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Mul),
                        Box::new(units_reduce_impl(env, *llhs)),
                        Box::new(units_reduce_impl(env, *rhs)),
                    )),
                    Box::new(units_reduce_impl(env, *lrhs)),
                ),
                // g*(s/m) => (g*s)/m
                (_, Node::BinOp(Token::Op(TokenOp::Div), rlhs, rrhs)) => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Mul),
                        Box::new(units_reduce_impl(env, *lhs)),
                        Box::new(units_reduce_impl(env, *rlhs)),
                    )),
                    Box::new(units_reduce_impl(env, *rrhs)),
                ),
                (_, _) => Node::BinOp(
                    Token::Op(TokenOp::Mul),
                    Box::new(units_reduce_impl(env, *lhs)),
                    Box::new(units_reduce_impl(env, *rhs)),
                ),
            },
            Token::Op(TokenOp::Div) => match (*lhs.clone(), *rhs.clone()) {
                // g/(m/s) => (g*s)/m
                (_, Node::BinOp(Token::Op(TokenOp::Div), rlhs, rrhs)) => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Mul),
                        Box::new(units_reduce_impl(env, *lhs)),
                        Box::new(units_reduce_impl(env, *rrhs)),
                    )),
                    Box::new(units_reduce_impl(env, *rlhs)),
                ),
                // (g/m)/s => g/(m*s)
                (Node::BinOp(Token::Op(TokenOp::Div), llhs, lrhs), _) => Node::BinOp(
                    Token::Op(TokenOp::Mul),
                    Box::new(units_reduce_impl(env, *llhs)),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Div),
                        Box::new(units_reduce_impl(env, *lrhs)),
                        Box::new(units_reduce_impl(env, *rhs)),
                    )),
                ),
                _ => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(units_reduce_impl(env, *lhs)),
                    Box::new(units_reduce_impl(env, *rhs)),
                ),
            },
            Token::Op(TokenOp::Caret) => {
                if let Node::Num(rhs_n, _) = *rhs {
                    if rhs_n == 2 {
                        // m^2 => m*m
                        Node::BinOp(
                            Token::Op(TokenOp::Mul),
                            Box::new(units_reduce_impl(env, *lhs.clone())),
                            Box::new(units_reduce_impl(env, *lhs)),
                        )
                    } else {
                        // m^n => m*m^(n-1)
                        Node::BinOp(
                            Token::Op(TokenOp::Mul),
                            Box::new(units_reduce_impl(
                                env,
                                Node::BinOp(
                                    Token::Op(TokenOp::Caret),
                                    lhs.clone(),
                                    Box::new(Node::Num(rhs_n - 1, Box::new(Node::None))),
                                ),
                            )),
                            Box::new(units_reduce_impl(env, *lhs)),
                        )
                    }
                } else {
                    Node::None
                }
            }
            _ => units,
        },
        _ => units,
    }
}

pub fn eval_units_reduce(env: &mut Env, original: Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_reduce {:?}\r", original);
    }
    match original.clone() {
        Node::Num(n, units) => {
            if let Node::Units(u) = *units {
                Node::Num(
                    n,
                    Box::new(Node::Units(Box::new(units_reduce_impl(env, *u)))),
                )
            } else {
                original
            }
        }
        Node::FNum(f, units) => {
            if let Node::Units(u) = *units {
                Node::FNum(
                    f,
                    Box::new(Node::Units(Box::new(units_reduce_impl(env, *u)))),
                )
            } else {
                original
            }
        }
        Node::CNum(n, units) => {
            if let Node::Units(u) = *units {
                Node::CNum(
                    n,
                    Box::new(Node::Units(Box::new(units_reduce_impl(env, *u)))),
                )
            } else {
                original
            }
        }
        _ => original,
    }
}

fn units_fraction_reduce(env: &mut Env, units: Node) -> Node {
    if env.is_debug() {
        eprintln!("units_fraction_reduce {:?}\r", units);
    }
    if let Node::UnitsFraction(mut numerator, mut denominator) = units {
        let mut new_nume = HashMap::new();
        for (nume_key, nume_value) in &mut numerator {
            if let Some(denom_value) = denominator.get(nume_key) {
                match (*nume_value).cmp(denom_value) {
                    Ordering::Greater => {
                        new_nume.insert(nume_key.to_string(), *nume_value - denom_value);
                        denominator.remove(nume_key);
                    }
                    Ordering::Less => {}
                    Ordering::Equal => {
                        denominator.remove(nume_key);
                    }
                }
            } else {
                new_nume.insert(nume_key.clone(), *nume_value);
            }
        }
        let mut new_denom = HashMap::new();
        for (denom_key, denom_value) in &mut denominator {
            if let Some(nume_value) = new_nume.get(denom_key) {
                match (*denom_value).cmp(nume_value) {
                    Ordering::Greater => {
                        new_denom.insert(denom_key.to_string(), *denom_value - nume_value);
                        new_nume.remove(denom_key);
                    }
                    Ordering::Less => {}
                    Ordering::Equal => {
                        new_nume.remove(denom_key);
                    }
                }
            } else {
                new_denom.insert(denom_key.clone(), *denom_value);
            }
        }
        Node::UnitsFraction(new_nume, new_denom)
    } else {
        units
    }
}

fn units_mul_to_hash<'a>(
    env: &Env<'a>,
    units: Node,
    hash: &'a mut HashMap<String, i32>,
) -> &'a mut HashMap<String, i32> {
    if env.is_debug() {
        eprintln!("units_mul_to_hash {:?}\r", units);
    }
    match units {
        Node::Num(_, _) => {
            hash.insert("_".to_string(), 1);
        }
        Node::Var(Token::Ident(u)) => {
            let count = hash.entry(u).or_insert(0);
            *count += 1;
        }
        Node::BinOp(Token::Op(TokenOp::Mul), lhs, rhs) => {
            units_mul_to_hash(env, *lhs, hash);
            units_mul_to_hash(env, *rhs, hash);
        }
        _ => {}
    }
    hash
}

pub fn eval_units_fraction(env: &mut Env, units: Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_fraction {:?}\r", units);
    }
    if let Node::BinOp(Token::Op(TokenOp::Div), nume, denom) = units.clone() {
        let mut hash_nume = HashMap::<String, i32>::new();
        let mut hash_denom = HashMap::<String, i32>::new();
        let numerator = units_mul_to_hash(env, *nume, &mut hash_nume);
        let denominator = units_mul_to_hash(env, *denom, &mut hash_denom);
        return units_fraction_reduce(
            env,
            Node::UnitsFraction(numerator.clone(), denominator.clone()),
        );
    } else if let Node::BinOp(Token::Op(TokenOp::Mul), _lhs, _rhs) = units.clone() {
        let mut hash_nume = HashMap::<String, i32>::new();
        let numerator = units_mul_to_hash(env, units, &mut hash_nume);
        return units_fraction_reduce(
            env,
            Node::UnitsFraction(numerator.clone(), HashMap::<String, i32>::new()),
        );
    } else if let Node::Var(Token::Ident(_)) = units {
        let mut hash_nume = HashMap::<String, i32>::new();
        let numerator = units_mul_to_hash(env, units, &mut hash_nume);
        return units_fraction_reduce(
            env,
            Node::UnitsFraction(numerator.clone(), HashMap::<String, i32>::new()),
        );
    } else if let Node::None = units {
        return units_fraction_reduce(
            env,
            Node::UnitsFraction(HashMap::<String, i32>::new(), HashMap::<String, i32>::new()),
        );
    }
    units_fraction_reduce(env, units)
}

pub fn eval_unit_prefix(env: &mut Env, units: &Node) -> (Node, bool) {
    if env.is_debug() {
        eprintln!("eval_unit_prefix {:?}\r", units);
    }
    match units {
        Node::Var(Token::Ident(unit_str)) => match unit_str.as_str() {
            // TODO: add more unit conversion
            "km" => (
                Node::FNum(
                    1000.0,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "m".to_owned(),
                    ))))),
                ),
                false,
            ),
            "mm" => (
                Node::FNum(
                    0.001,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "m".to_owned(),
                    ))))),
                ),
                false,
            ),
            "mi" => (
                // 1 mile = 1.6 km = 1600 m
                Node::FNum(
                    1600.0,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "m".to_owned(),
                    ))))),
                ),
                false,
            ),
            "in" => (
                // 1 inch = 25.4 mm = 0.0254 m
                Node::FNum(
                    0.0254,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "m".to_owned(),
                    ))))),
                ),
                false,
            ),
            "feet" => (
                // 1 feet = 12 inch = 30.48 cm
                Node::FNum(
                    12.0,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "in".to_owned(),
                    ))))),
                ),
                false,
            ),
            _ => (Node::Num(1, Box::new(units.clone())), true),
        },
        Node::BinOp(op, lhs, rhs) => {
            let (left_node, final_left) = eval_unit_prefix(env, lhs);
            let (right_node, final_right) = eval_unit_prefix(env, rhs);
            let new_node = eval_binop(
                env,
                &Node::BinOp(op.clone(), Box::new(left_node), Box::new(right_node)),
            );
            (new_node.unwrap(), final_left && final_right)
        }
        Node::Num(_n, _u) => (units.clone(), true),
        Node::FNum(_n, _u) => (units.clone(), true),
        _ => (Node::Num(1, Box::new(units.clone())), true),
    }
}

pub fn units_fraction_to_string(
    numerator: &HashMap<String, i32>,
    denominator: &HashMap<String, i32>,
) -> String {
    let mut nume_vec: Vec<(&String, &i32)> = numerator.iter().collect();
    nume_vec.sort_by(|a, b| a.0.cmp(b.0));

    let mut denom_vec: Vec<(&String, &i32)> = denominator.iter().collect();
    denom_vec.sort_by(|a, b| a.0.cmp(b.0));

    format!("{:?}/{:?}", nume_vec, denom_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_as_string(env: &mut Env, input: &str) -> String {
        let node = parse(env, &(lexer(input.to_owned())).unwrap()).unwrap();
        let node = eval(env, &node).unwrap();
        match node {
            Node::Num(num, ref u) => match &**u {
                Node::Units(un) => match &**un {
                    Node::UnitsFraction(a, b) => {
                        let units_str = units_fraction_to_string(a, b);
                        format!("Num({}, {})", num, units_str)
                    }
                    _ => format!("{:?}", node),
                },
                _ => format!("{:?}", node),
            },
            _ => format!("{:?}", node),
        }
    }

    #[test]
    fn test_eval_units() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(
            eval_as_string(&mut env, "1[m]"),
            "Num(1, [(\"m\", 1)]/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1[1/m]"),
            "Num(1, [(\"_\", 1)]/[(\"m\", 1)])"
        );
        assert_eq!(
            eval_as_string(&mut env, "2[m*s]"),
            "Num(2, [(\"m\", 1), (\"s\", 1)]/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1[m]*2[s]"),
            "Num(2, [(\"m\", 1), (\"s\", 1)]/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3[m/s]"),
            "Num(3, [(\"m\", 1)]/[(\"s\", 1)])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "6[m]/2[s]"),
            "Num(3, [(\"m\", 1)]/[(\"s\", 1)])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "6[m*m]/2[s]"),
            "Num(3, [(\"m\", 2)]/[(\"s\", 1)])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3*2[m]"),
            eval_as_string(&mut env, "6[m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "2[m]*3"),
            eval_as_string(&mut env, "6[m]")
        );
        // unit reduction
        assert_eq!(
            eval_as_string(&mut env, "6[m^2]/3[s]"),
            eval_as_string(&mut env, "6[m*m]/3[s]")
        );
        assert_eq!(
            eval_as_string(&mut env, "18[m^3]/2[s]"),
            eval_as_string(&mut env, "18[m*m*m]/2[s]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6[g]/2[m/s]"),
            eval_as_string(&mut env, "3[g*s/m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6[g/m]*2[s]"),
            eval_as_string(&mut env, "12[g*s/m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "2[g]*6[s/m]"),
            eval_as_string(&mut env, "12[g*s/m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6[g/m]/2[s]"),
            eval_as_string(&mut env, "3[g/m/s]")
        );
        // unit expand
        // TODO: to be added more conversion
        assert_eq!(
            eval_as_string(&mut env, "6[km]"),
            eval_as_string(&mut env, "6000.0[m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6000[mm]"),
            eval_as_string(&mut env, "6.0[m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "0.001[1/m]"),
            eval_as_string(&mut env, "1.0[1/km]"),
        );
        assert_eq!(
            eval_as_string(&mut env, "3"),
            eval_as_string(&mut env, "3[m/m]"),
        );
        assert_eq!(
            eval_as_string(&mut env, "1[mi]"),
            eval_as_string(&mut env, "1600.0[m]"),
        );
        assert_eq!(
            eval_as_string(&mut env, "1[in]"),
            eval_as_string(&mut env, "25.4[mm]"),
        );
        assert_eq!(
            eval_as_string(&mut env, "1[feet]"),
            eval_as_string(&mut env, "12.0[in]"),
        );
    }
}
