use super::*;

fn eval_as_string(env: &mut Env, input: &str) -> String {
    match parse(env, &(lexer(input.to_owned())).unwrap()) {
        Ok(n) => {
            let n = eval(env, &n).unwrap();
            format!("{:?}", n)
        }
        Err(e) => {
            format!("{}", e)
        }
    }
}

pub fn run_test(env: &mut Env) {
    println!("lexer");
    println!("0xa -> {:?}", lexer("0xa".to_owned()).unwrap());
    println!("011 -> {:?}", lexer("011".to_owned()).unwrap());
    println!("0b11 -> {:?}", lexer("0b11".to_owned()).unwrap());
    println!("1.1 -> {:?}", lexer("1.1".to_owned()).unwrap());
    println!("0.1 -> {:?}", lexer("0.1".to_owned()).unwrap());
    println!("1 -> {:?}", lexer("1".to_owned()).unwrap());
    println!("0 -> {:?}", lexer("0".to_owned()).unwrap());
    println!("10 1 -> {:?}", lexer("10 1".to_owned()).unwrap());
    println!("1+1 -> {:?}", lexer("1+1".to_owned()).unwrap());
    println!("1-1 -> {:?}", lexer("1-1".to_owned()).unwrap());
    println!("-1 -> {:?}", lexer("-1".to_owned()).unwrap());
    println!(
        "+-*/%()^100 -> {:?}",
        lexer("+-*/%()^-100".to_owned()).unwrap()
    );
    println!("1.234 -> {:?}", lexer("1.234".to_owned()).unwrap());
    println!("1.234e-56 -> {:?}", lexer("1.234e-56".to_owned()).unwrap());
    println!(
        "-1.234e-56-78 -> {:?}",
        lexer("-1.234e-56-78".to_owned()).unwrap()
    );
    println!(
        "1/(2*3.14*270e-12*31.4e3) -> {:?}",
        lexer("1/(2*3.14*270e-12*31.4e3)".to_owned()).unwrap()
    );
    match lexer("0b12".to_owned()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
    match lexer("018".to_owned()) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }

    println!();
    println!("parser");
    println!(
        "1 -> {:?}",
        parse(env, &(lexer("1".to_owned())).unwrap()).unwrap()
    );
    println!(
        "0 -> {:?}",
        parse(env, &(lexer("0".to_owned())).unwrap()).unwrap()
    );
    println!(
        "-1 -> {:?}",
        parse(env, &(lexer("-1".to_owned())).unwrap()).unwrap()
    );
    println!(
        "9223372036854775807 -> {:?}",
        parse(env, &(lexer("9223372036854775807".to_owned()).unwrap())).unwrap()
    );
    println!(
        "-9223372036854775808 -> {:?}",
        parse(env, &(lexer("-9223372036854775808".to_owned()).unwrap())).unwrap()
    );
    println!(
        "1+2 -> {:?}",
        parse(env, &(lexer("1+2".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1-2 -> {:?}",
        parse(env, &(lexer("1-2".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1+-2 -> {:?}",
        parse(env, &(lexer("1+-2".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1*2 -> {:?}",
        parse(env, &(lexer("1*2".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1*2+3 -> {:?}",
        parse(env, &(lexer("1*2+3".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1+2*3 -> {:?}",
        parse(env, &(lexer("1+2*3".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1*(2+3) -> {:?}",
        parse(env, &(lexer("1*(2+3)".to_owned())).unwrap()).unwrap()
    );
    println!(
        "(1+2)*3 -> {:?}",
        parse(env, &(lexer("(1+2)*3".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1+2+3 -> {:?}",
        parse(env, &(lexer("1+2+3".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1*2*3 -> {:?}",
        parse(env, &(lexer("1*2*3".to_owned())).unwrap()).unwrap()
    );
    println!(
        "-(1+2) -> {:?}",
        parse(env, &(lexer("-(1+2)".to_owned())).unwrap()).unwrap()
    );
    println!(
        "1.2*3.4e5 -> {:?}",
        parse(env, &(lexer("1.2*-3.4e5 ".to_owned()).unwrap())).unwrap()
    );
    println!(
        "1/(2*3.14*270e-12*31.4e3) -> {:?}",
        parse(
            env,
            &(lexer("1/(2*3.14*270e-12*31.4e3)".to_owned()).unwrap())
        )
        .unwrap()
    );
    println!(
        "1+2+ -> {:?}",
        parse(env, &(lexer("1+2+".to_owned()).unwrap()))
    );

    println!();
    println!("eval");
    println!("1 -> {:?}", eval_as_string(env, "1"));
    println!("0 -> {:?}", eval_as_string(env, "0"));
    println!("-1 -> {:?}", eval_as_string(env, "-1"));
    println!(
        "9223372036854775807 -> {:?}",
        eval_as_string(env, "9223372036854775807")
    );
    println!(
        "-9223372036854775807 -> {:?}",
        eval_as_string(env, "-9223372036854775807")
    );
    println!("1+2 -> {:?}", eval_as_string(env, "1+2"));
    println!("1-2 -> {:?}", eval_as_string(env, "1-2"));
    println!("1+-2 -> {:?}", eval_as_string(env, "1+-2"));
    println!("1*2 -> {:?}", eval_as_string(env, "1*2"));
    println!("1*2+3 -> {:?}", eval_as_string(env, "1*2+3"));
    println!("1+2*3 -> {:?}", eval_as_string(env, "1+2*3"));
    println!("1*(2+3) -> {:?}", eval_as_string(env, "1*(2+3)"));
    println!("(1+2)*3 -> {:?}", eval_as_string(env, "(1+2)*3"));
    println!("1+2+3 -> {:?}", eval_as_string(env, "1+2+3"));
    println!("1*2*3 -> {:?}", eval_as_string(env, "1*2*3"));
    println!("(1+2)*(3+4) -> {:?}", eval_as_string(env, "(1+2)*(3+4)"));
    println!("1.1*2*3 -> {:?}", eval_as_string(env, "1.1*2*3"));
    println!(
        "1/(2*3.14*270e-12*31.4e3) -> {:?}",
        eval_as_string(env, "1/(2*3.14*270e-12*31.4e3)")
    );
    println!("-(1+2) -> {:?}", eval_as_string(env, "-(1+2)"));
    println!(
        "1/(2*pi*10k*4.7u) -> {:?}",
        eval_as_string(env, "1/(2*pi*10k*4.7u)")
    );
    println!("sin(pi/2) -> {:?}", eval_as_string(env, "sin(pi/2)"));
    println!("abs(-2) -> {:?}", eval_as_string(env, "abs(-2)"));
    println!("1+2+ -> {:?}", eval_as_string(env, "1+2+"));
    println!("1+2(3+4) -> {:?}", eval_as_string(env, "1+2(3+4)"));
}
