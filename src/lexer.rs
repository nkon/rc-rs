use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenOp {
    Plus,       // +
    Minus,      // -
    Mul,        // *
    Div,        // /
    Mod,        // %
    Para,       // //
    ParenLeft,  // (
    ParenRight, // )
    Caret,      // ^
    Comma,      // ,
    Equal,      // =
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i128),
    FNum(f64),
    Op(TokenOp),
    Ident(String),
}

/// Cut out sequence of num_char as `String` from input `chars: &[char]`.
/// Increment index and return as a member of tuple.
fn tok_get_num(chars: &[char], index: usize) -> (String, usize) {
    let mut i = index;
    if i < chars.len() {
        match chars[i] {
            '-' | '0'..='9' => {
                // '-' is required for parsing exponent of floating point number format.
                let mut ret = String::from(chars[i]);
                i += 1;
                while i < chars.len() {
                    match chars[i] {
                        '0'..='9' => {
                            ret.push(chars[i]);
                            i += 1;
                        }
                        _ => {
                            return (ret, i);
                        }
                    }
                }
                (ret, i)
            }
            _ => (String::from(""), i),
        }
    } else {
        (String::from(""), i)
    }
}

/// Eat integer numbers from input array.
/// Return `Token::Num()` with `Result<,Err(String)>`.
/// Increment index and return as a member of tuple.
fn tok_num_int(chars: &[char], index: usize) -> Result<(Token, usize), MyError> {
    let mut i = index;
    let radix: u32;
    let mut mantissa = String::from("0");
    let mut err_str = String::from("0");

    if i < chars.len() {
        match chars[i] {
            'x' | 'X' => {
                radix = 16;
                err_str.push(chars[i]);
                i += 1;
            }
            'b' | 'B' => {
                radix = 2;
                err_str.push(chars[i]);
                i += 1;
            }
            '0'..='7' => {
                radix = 8;
            }
            _ => {
                return Ok((Token::Num(0), i));
            }
        }
    } else {
        return Ok((Token::Num(0), i));
    }

    while i < chars.len() {
        match chars[i] {
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                mantissa.push(chars[i]);
                err_str.push(chars[i]);
                i += 1;
            }
            '_' => {
                i += 1;
            }
            _ => {
                break;
            }
        }
    }

    match i128::from_str_radix(&mantissa, radix) {
        Ok(int) => Ok((Token::Num(int), i)),
        Err(e) => Err(MyError::LexerIntError(err_str, e)),
    }
}

/// Eat numbers from input array.
/// Foreword to `tok_num_int()` when integer, i.e. decimal, hexadecimal, octal or binary.
/// Return `Token::Num()` or `Token::FNum()` with `Result<,Err(String)>`.
/// Increment index and return as a member of tuple.
fn tok_num(chars: &[char], index: usize) -> Result<(Token, usize), MyError> {
    let mut i = index;
    let mut mantissa = String::new();
    let mut exponent = String::new();
    let mut has_dot = false;
    let mut has_exponent = false;
    if chars[i] == '0' {
        if (i + 1) < chars.len() {
            i += 1;
            match chars[i] {
                '0'..='9' | 'a'..='f' | 'A'..='F' | 'x' | 'X' => {
                    return tok_num_int(chars, i);
                }
                '.' => {
                    mantissa.push('0');
                    mantissa.push(chars[i]);
                    has_dot = true;
                    i += 1;
                }
                _ => {
                    return Ok((Token::Num(0), i));
                }
            }
        } else {
            return Ok((Token::Num(0), i + 1));
        }
    }
    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                mantissa.push(chars[i]);
                i += 1;
            }
            '_' => {
                // separator
                i += 1;
            }
            '.' => {
                mantissa.push(chars[i]);
                i += 1;
                has_dot = true;
            }
            'e' | 'E' => {
                i += 1;
                has_dot = true; // no dot but move to floating mode.
                has_exponent = true;
                if i < chars.len() {
                    let (a, b) = tok_get_num(chars, i);
                    exponent = a;
                    i = b;
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
    if !has_dot {
        match mantissa.parse::<i128>() {
            Ok(int) => {
                return Ok((Token::Num(int), i));
            }
            Err(e) => {
                return Err(MyError::LexerIntError(mantissa, e));
            }
        }
    }
    if has_exponent {
        mantissa.push('e');
        mantissa.push_str(&exponent);
    }
    // Ok((Token::FNum(mantissa.parse::<f64>()?), i))
    match mantissa.parse::<f64>() {
        Ok(float) => Ok((Token::FNum(float), i)),
        Err(e) => Err(MyError::LexerFloatError(mantissa, e)),
    }
}

fn tok_ident(chars: &[char], index: usize) -> (Token, usize) {
    let mut i = index;
    let mut ret = String::new();
    while i < chars.len() {
        match chars[i] {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                ret.push(chars[i]);
                i += 1;
            }
            _ => {
                return (Token::Ident(ret), i);
            }
        }
    }
    (Token::Ident(ret), i)
}

/// Input: `String`
/// Output: `Result<Vec<Token>, String>`
///
/// # Examples
/// ```
/// use rc::lexer;
/// use rc::Token;
/// assert_eq!(lexer("1".to_owned()).unwrap(), [Token::Num(1)]);
/// assert_eq!(lexer("0".to_owned()).unwrap(), [Token::Num(0)]);
/// assert_eq!(lexer("10".to_owned()).unwrap(), [Token::Num(10)]);
/// assert_eq!(lexer("1.1".to_owned()).unwrap(), [Token::FNum(1.1)]);
/// assert_eq!(lexer("0.1".to_owned()).unwrap(), [Token::FNum(0.1)]);
/// assert_eq!(lexer("1.1E2".to_owned()).unwrap(), [Token::FNum(110.0)]);
/// assert_eq!(lexer("1.1E-2".to_owned()).unwrap(), [Token::FNum(0.011)]);
/// assert_eq!(lexer("100_000".to_owned()).unwrap(), [Token::Num(100000)]);
/// assert_eq!(lexer("0xa".to_owned()).unwrap(), [Token::Num(10)]);
/// assert_eq!(lexer("011".to_owned()).unwrap(), [Token::Num(9)]);
/// assert_eq!(lexer("0b11".to_owned()).unwrap(), [Token::Num(3)]);
/// assert_eq!(lexer("1e3".to_owned()).unwrap(), [Token::FNum(1000.0)]);
/// assert_eq!(lexer("9223372036854775807".to_owned()).unwrap(), [Token::Num(9223372036854775807)]);
/// assert_eq!(lexer("18446744073709551615".to_owned()).unwrap(), [Token::Num(18446744073709551615)]);
/// ```
pub fn lexer(s: String) -> Result<Vec<Token>, MyError> {
    let mut ret = Vec::new();

    let chars: Vec<char> = s.chars().collect();
    let mut i: usize = 0;
    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                // `Num` or `FNum` begin from '0'..='9'.
                let (tk, j) = tok_num(&chars, i)?;
                i = j;
                ret.push(tk);
            }
            '+' => {
                ret.push(Token::Op(TokenOp::Plus));
                i += 1;
            }
            '-' => {
                ret.push(Token::Op(TokenOp::Minus));
                i += 1;
            }
            '*' => {
                ret.push(Token::Op(TokenOp::Mul));
                i += 1;
            }
            '/' => {
                if i + 1 < chars.len() && chars[i + 1] == '/' {
                    ret.push(Token::Op(TokenOp::Para));
                    i += 2;
                } else {
                    ret.push(Token::Op(TokenOp::Div));
                    i += 1;
                }
            }
            '%' => {
                ret.push(Token::Op(TokenOp::Mod));
                i += 1;
            }
            '(' => {
                ret.push(Token::Op(TokenOp::ParenLeft));
                i += 1;
            }
            ')' => {
                ret.push(Token::Op(TokenOp::ParenRight));
                i += 1;
            }
            '^' => {
                ret.push(Token::Op(TokenOp::Caret));
                i += 1;
            }
            ',' => {
                ret.push(Token::Op(TokenOp::Comma));
                i += 1;
            }
            '=' => {
                ret.push(Token::Op(TokenOp::Equal));
                i += 1;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let (tk, j) = tok_ident(&chars, i);
                i = j;
                ret.push(tk);
            }
            '#' => {
                return Ok(ret);
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s2v(str: &str) -> Vec<char> {
        str.chars().collect::<Vec<char>>()
    }

    fn assert_tok_index(result: Result<(lexer::Token, usize), MyError>, tok: Token, index: usize) {
        if let Ok((good, idx)) = result {
            assert_eq!(good, tok);
            assert_eq!(idx, index);
        }
    }

    #[test]
    fn test_tok_get_num() {
        assert_eq!(tok_get_num(&s2v("0"), 0), ("0".to_owned(), 1));
        assert_eq!(tok_get_num(&s2v("1"), 0), ("1".to_owned(), 1));
        assert_eq!(tok_get_num(&s2v("34"), 0), ("34".to_owned(), 2));
        assert_eq!(tok_get_num(&s2v("56a"), 0), ("56".to_owned(), 2));
        assert_eq!(tok_get_num(&s2v(""), 0), ("".to_owned(), 0));
        assert_eq!(tok_get_num(&s2v("a"), 0), ("".to_owned(), 0));
    }
    #[test]
    fn test_tok_num_int() {
        assert_tok_index(tok_num_int(&s2v("0x1"), 1), Token::Num(1), 3);
        assert_tok_index(tok_num_int(&s2v("0xa"), 1), Token::Num(10), 3);
        assert_tok_index(tok_num_int(&s2v("0x10"), 1), Token::Num(16), 4);
        assert_tok_index(tok_num_int(&s2v("0b10"), 1), Token::Num(2), 4);
        assert_tok_index(tok_num_int(&s2v("0b1_0"), 1), Token::Num(2), 5);
        assert_tok_index(tok_num_int(&s2v("010"), 1), Token::Num(8), 3);
    }
    #[test]
    fn test_tok_num() {
        assert_tok_index(tok_num(&s2v("0x1"), 0), Token::Num(1), 3);
        assert_tok_index(tok_num(&s2v("0xa"), 0), Token::Num(10), 3);
        assert_tok_index(tok_num(&s2v("0x10"), 0), Token::Num(16), 4);
        assert_tok_index(tok_num(&s2v("0b10"), 0), Token::Num(2), 4);
        assert_tok_index(tok_num(&s2v("0b1_0"), 0), Token::Num(2), 5);
        assert_tok_index(tok_num(&s2v("010"), 0), Token::Num(8), 3);
        assert_tok_index(tok_num(&s2v("10"), 0), Token::Num(10), 2);
        assert_tok_index(tok_num(&s2v("10.1"), 0), Token::FNum(10.1), 4);
        assert_tok_index(tok_num(&s2v("10e1"), 0), Token::FNum(100.0), 4);
        assert_tok_index(tok_num(&s2v("10e1+"), 0), Token::FNum(100.0), 4);
        assert_tok_index(tok_num(&s2v("1"), 0), Token::Num(1), 1);
        assert_tok_index(tok_num(&s2v("0"), 0), Token::Num(0), 1);
        assert_tok_index(tok_num(&s2v("10"), 0), Token::Num(10), 2);
        assert_tok_index(tok_num(&s2v("1.1"), 0), Token::FNum(1.1), 3);
        assert_tok_index(tok_num(&s2v("0.1"), 0), Token::FNum(0.1), 3);
        assert_tok_index(tok_num(&s2v("1.1E2"), 0), Token::FNum(110.0), 5);
        assert_tok_index(tok_num(&s2v("1.1E-2"), 0), Token::FNum(0.011), 6);
        assert_tok_index(tok_num(&s2v("100_000"), 0), Token::Num(100000), 7);
        assert_tok_index(tok_num(&s2v("0xa"), 0), Token::Num(10), 3);
        assert_tok_index(tok_num(&s2v("011"), 0), Token::Num(9), 3);
        assert_tok_index(tok_num(&s2v("0b11"), 0), Token::Num(3), 4);
        assert_tok_index(tok_num(&s2v("1e3"), 1), Token::FNum(1000.0), 3);
        assert_tok_index(
            tok_num(&s2v("9223372036854775807"), 0),
            Token::Num(9223372036854775807),
            19,
        );
        assert_tok_index(
            tok_num(&s2v("18446744073709551615"), 0),
            Token::Num(18446744073709551615),
            20,
        );
    }

    #[test]
    fn test_tok_num_error() {
        if let Ok(_) = lexer("018".to_owned()) {
            assert!(false);
        }
        if let Ok(_) = lexer("0b12".to_owned()) {
            assert!(false);
        }
    }

    #[test]
    fn test_tok_ident() {
        let (tok, idx) = tok_ident(&s2v("i"), 0);
        assert_eq!(tok, Token::Ident("i".to_owned()));
        assert_eq!(idx, 1);
        let (tok, idx) = tok_ident(&s2v("sin()"), 0);
        assert_eq!(tok, Token::Ident("sin".to_owned()));
        assert_eq!(idx, 3);
    }
    #[test]
    fn test_lexer() {
        assert_eq!(
            lexer("1+2+3".to_owned()).unwrap(),
            [
                Token::Num(1),
                Token::Op(TokenOp::Plus),
                Token::Num(2),
                Token::Op(TokenOp::Plus),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer(" 1 + 2 + 3 ".to_owned()).unwrap(),
            [
                Token::Num(1),
                Token::Op(TokenOp::Plus),
                Token::Num(2),
                Token::Op(TokenOp::Plus),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer(" 1 + 2 + 3 ### comment".to_owned()).unwrap(),
            [
                Token::Num(1),
                Token::Op(TokenOp::Plus),
                Token::Num(2),
                Token::Op(TokenOp::Plus),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer("1 2 34+-*/%()-^".to_owned()).unwrap(),
            [
                Token::Num(1),
                Token::Num(2),
                Token::Num(34),
                Token::Op(TokenOp::Plus),
                Token::Op(TokenOp::Minus),
                Token::Op(TokenOp::Mul),
                Token::Op(TokenOp::Div),
                Token::Op(TokenOp::Mod),
                Token::Op(TokenOp::ParenLeft),
                Token::Op(TokenOp::ParenRight),
                Token::Op(TokenOp::Minus),
                Token::Op(TokenOp::Caret)
            ]
        );
        assert_eq!(
            lexer("sin(2.0)".to_owned()).unwrap(),
            [
                Token::Ident("sin".to_owned()),
                Token::Op(TokenOp::ParenLeft),
                Token::FNum(2.0),
                Token::Op(TokenOp::ParenRight),
            ]
        );
        assert_eq!(
            lexer("1k*3.0u".to_owned()).unwrap(),
            [
                Token::Num(1),
                Token::Ident("k".to_owned()),
                Token::Op(TokenOp::Mul),
                Token::FNum(3.0),
                Token::Ident("u".to_owned()),
            ]
        );
        assert_eq!(
            lexer("sin(0.5*pi)".to_owned()).unwrap(),
            [
                Token::Ident("sin".to_owned()),
                Token::Op(TokenOp::ParenLeft),
                Token::FNum(0.5),
                Token::Op(TokenOp::Mul),
                Token::Ident("pi".to_owned()),
                Token::Op(TokenOp::ParenRight),
            ]
        );
        assert_eq!(
            lexer("123    #asdfasdfasfd".to_owned()).unwrap(),
            [Token::Num(123)]
        );
        assert_eq!(
            lexer("a=1".to_owned()).unwrap(),
            [
                Token::Ident("a".to_owned()),
                Token::Op(TokenOp::Equal),
                Token::Num(1),
            ]
        );
    }
}
