use super::*;

pub fn run_test() {
    println!("lexer");
    println!("0xa -> {:?}", lexer("0xa".to_string()));
    println!("011 -> {:?}", lexer("011".to_string()));
    println!("0b11 -> {:?}", lexer("0b11".to_string()));
    println!("1.1 -> {:?}", lexer("1.1".to_string()));
    println!("0.1 -> {:?}", lexer("0.1".to_string()));
    println!("1 -> {:?}", lexer("1".to_string()));
    println!("0 -> {:?}", lexer("0".to_string()));
    println!("10 1 -> {:?}", lexer("10 1".to_string()));
    println!("1+1 -> {:?}", lexer("1+1".to_string()));
    println!("1-1 -> {:?}", lexer("1-1".to_string()));
    println!("-1 -> {:?}", lexer("-1".to_string()));
    println!("+-*/%()^100 -> {:?}", lexer("+-*/%()^-100".to_string()));
    println!("1.234 -> {:?}", lexer("1.234".to_string()));
    println!("1.234e-56 -> {:?}", lexer("1.234e-56".to_string()));
    println!("-1.234e-56-78 -> {:?}", lexer("-1.234e-56-78".to_string()));
    println!(
        "1/(2*3.14*270e-12*31.4e3) -> {:?}",
        lexer("1/(2*3.14*270e-12*31.4e3)".to_string())
    );
    lexer("0b12".to_string());
    lexer("018".to_string());

    println!("");
    println!("parser");
    println!("1 -> {:?}", parse(&lexer("1".to_string())));
    println!("0 -> {:?}", parse(&lexer("0".to_string())));
    println!("-1 -> {:?}", parse(&lexer("-1".to_string())));
    println!(
        "9223372036854775807 -> {:?}",
        parse(&lexer("9223372036854775807".to_string()))
    );
    println!(
        "-9223372036854775808 -> {:?}",
        parse(&lexer("-9223372036854775808".to_string()))
    );
    println!("1+2 -> {:?}", parse(&lexer("1+2".to_string())));
    println!("1-2 -> {:?}", parse(&lexer("1-2".to_string())));
    println!("1+-2 -> {:?}", parse(&lexer("1+-2".to_string())));
    println!("1*2 -> {:?}", parse(&lexer("1*2".to_string())));
    println!("1*2+3 -> {:?}", parse(&lexer("1*2+3".to_string())));
    println!("1+2*3 -> {:?}", parse(&lexer("1+2*3".to_string())));
    println!("1*(2+3) -> {:?}", parse(&lexer("1*(2+3)".to_string())));
    println!("(1+2)*3 -> {:?}", parse(&lexer("(1+2)*3".to_string())));
    println!("1+2+3 -> {:?}", parse(&lexer("1+2+3".to_string())));
    println!("1*2*3 -> {:?}", parse(&lexer("1*2*3".to_string())));
    println!("-(1+2) -> {:?}", parse(&lexer("-(1+2)".to_string())));
    println!(
        "1.2*3.4e5 -> {:?}",
        parse(&lexer("1.2*-3.4e5 ".to_string()))
    );
    println!(
        "1/(2*3.14*270e-12*31.4e3) -> {:?}",
        parse(&lexer("1/(2*3.14*270e-12*31.4e3)".to_string()))
    );
    println!("");
    println!("eval");
    println!("1 -> {:?}", eval(&parse(&lexer("1".to_string()))));
    println!("0 -> {:?}", eval(&parse(&lexer("0".to_string()))));
    println!("-1 -> {:?}", eval(&parse(&lexer("-1".to_string()))));
    println!(
        "9223372036854775807 -> {:?}",
        eval(&parse(&lexer("9223372036854775807".to_string())))
    );
    println!(
        "-9223372036854775807 -> {:?}",
        eval(&parse(&lexer("-9223372036854775807".to_string())))
    );
    println!("1+2 -> {:?}", eval(&parse(&lexer("1+2".to_string()))));
    println!("1-2 -> {:?}", eval(&parse(&lexer("1-2".to_string()))));
    println!("1+-2 -> {:?}", eval(&parse(&lexer("1+-2".to_string()))));
    println!("1*2 -> {:?}", eval(&parse(&lexer("1*2".to_string()))));
    println!("1*2+3 -> {:?}", eval(&parse(&lexer("1*2+3".to_string()))));
    println!("1+2*3 -> {:?}", eval(&parse(&lexer("1+2*3".to_string()))));
    println!(
        "1*(2+3) -> {:?}",
        eval(&parse(&lexer("1*(2+3)".to_string())))
    );
    println!(
        "(1+2)*3 -> {:?}",
        eval(&parse(&lexer("(1+2)*3".to_string())))
    );
    println!("1+2+3 -> {:?}", eval(&parse(&lexer("1+2+3".to_string()))));
    println!("1*2*3 -> {:?}", eval(&parse(&lexer("1*2*3".to_string()))));
    println!(
        "(1+2)*(3+4) -> {:?}",
        eval(&parse(&lexer("(1+2)*(3+4)".to_string())))
    );
    println!(
        "1.1*2*3 -> {:?}",
        eval(&parse(&lexer("1.1*2*3".to_string())))
    );
    println!(
        "1/(2*3.14*270e-12*31.4e3) -> {:?}",
        eval(&parse(&lexer("1/(2*3.14*270e-12*31.4e3)".to_string())))
    );
    println!("-(1+2) -> {:?}", eval(&parse(&lexer("-(1+2)".to_string()))));
}
