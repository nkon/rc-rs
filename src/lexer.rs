use super::*;

// TODO: add Doc-test.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i128),
    FNum(f64),
    Op(char),
    Ident(String),
}

fn tok_get_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    match c {
        '-' | '0'..='9' => {
            let mut ret = String::from(c);
            iter.next();
            while let Some(&c) = iter.peek() {
                match c {
                    '0'..='9' => {
                        ret.push(c);
                        iter.next();
                    }
                    _ => {
                        return ret;
                    }
                }
            }
            ret
        }
        _ => String::from('0'),
    }
}

fn tok_num_int<T: Iterator<Item = char>>(
    _c: char,
    iter: &mut Peekable<T>,
) -> Result<Token, String> {
    let mut radix = 10;
    let mut mantissa = String::from("0");
    let mut err_str = String::from("0");

    if let Some(&c) = iter.peek() {
        match c {
            'x' | 'X' => {
                radix = 16;
                iter.next();
                err_str.push(c);
            }
            'b' | 'B' => {
                radix = 2;
                iter.next();
                err_str.push(c);
            }
            '0'..='7' => {
                radix = 8;
            }
            _ => {
                return Ok(Token::Num(0));
            }
        }
    }
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                mantissa.push(c);
                err_str.push(c);
                iter.next();
            }
            '_' => {
                iter.next();
            }
            _ => {
                break;
            }
        }
    }
    match i128::from_str_radix(&mantissa, radix) {
        Ok(int) => Ok(Token::Num(int)),
        Err(e) => Err(format!("Error: Integer format: {} {}", e, err_str)),
    }
}

fn tok_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Result<Token, String> {
    let mut mantissa = String::from(c);
    let mut exponent = String::new();
    let mut has_dot = false;
    let mut has_exponent = false;
    if mantissa == "0" {
        match iter.peek() {
            Some(&c) => match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' | 'x' | 'X' => {
                    return tok_num_int(c, iter);
                }
                _ => {}
            },
            None => {
                return Ok(Token::Num(0));
            }
        }
    }
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                mantissa.push(c);
                iter.next();
            }
            '_' => {
                iter.next();
            }
            '.' => {
                mantissa.push(c);
                iter.next();
                has_dot = true;
            }
            'e' | 'E' => {
                iter.next();
                has_dot = true; // no dot but move to floating mode.
                has_exponent = true;
                let &c = iter.peek().unwrap();
                exponent = tok_get_num(c, iter);
                break;
            }
            _ => {
                break;
            }
        }
    }
    if !has_dot {
        match mantissa.parse::<i128>() {
            Ok(int) => {
                return Ok(Token::Num(int));
            }
            Err(e) => {
                return Err(format!("Error: Integer format: {} {}", e, mantissa));
            }
        }
    }
    if has_exponent {
        mantissa.push('e');
        mantissa.push_str(&exponent);
    }
    match mantissa.parse::<f64>() {
        Ok(float) => Ok(Token::FNum(float)),
        Err(e) => Err(format!("Error: Float format: {} {}", e, mantissa)),
    }
}

/// # Examples
/// ```
/// use rc::lexer;
/// use rc::Token;
/// assert_eq!(lexer("1".to_string()).unwrap(), [Token::Num(1)]);
/// assert_eq!(lexer("0".to_string()).unwrap(), [Token::Num(0)]);
/// assert_eq!(lexer("10".to_string()).unwrap(), [Token::Num(10)]);
/// assert_eq!(lexer("1.1".to_string()).unwrap(), [Token::FNum(1.1)]);
/// assert_eq!(lexer("0.1".to_string()).unwrap(), [Token::FNum(0.1)]);
/// assert_eq!(lexer("1.1E2".to_string()).unwrap(), [Token::FNum(110.0)]);
/// assert_eq!(lexer("1.1E-2".to_string()).unwrap(), [Token::FNum(0.011)]);
/// assert_eq!(lexer("100_000".to_string()).unwrap(), [Token::Num(100000)]);
/// assert_eq!(lexer("0xa".to_string()).unwrap(), [Token::Num(10)]);
/// assert_eq!(lexer("011".to_string()).unwrap(), [Token::Num(9)]);
/// assert_eq!(lexer("0b11".to_string()).unwrap(), [Token::Num(3)]);
/// assert_eq!(lexer("1e3".to_string()).unwrap(), [Token::FNum(1000.0)]);
/// assert_eq!(lexer("9223372036854775807".to_string()).unwrap(), [Token::Num(9223372036854775807)]);
/// assert_eq!(lexer("18446744073709551615".to_string()).unwrap(), [Token::Num(18446744073709551615)]);
/// ```
// TODO: change from peekable iterator to Vec and index.
// TODO: handle vars/functions.
pub fn lexer(s: String) -> Result<Vec<Token>, String> {
    let mut ret = Vec::new();

    let mut iter = s.chars().peekable();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                iter.next();
                let tk = tok_num(c, &mut iter);
                match tk {
                    Ok(tk) => {
                        ret.push(tk);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            '+' | '-' | '*' | '/' | '%' | '(' | ')' | '^' => {
                iter.next();
                ret.push(Token::Op(c));
            }
            _ => {
                let _ = iter.next();
            }
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        assert_eq!(
            lexer("1+2+3".to_string()).unwrap(),
            [
                Token::Num(1),
                Token::Op('+'),
                Token::Num(2),
                Token::Op('+'),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer(" 1 + 2 + 3 ".to_string()).unwrap(),
            [
                Token::Num(1),
                Token::Op('+'),
                Token::Num(2),
                Token::Op('+'),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer("1 2 34+-*/%()-^".to_string()).unwrap(),
            [
                Token::Num(1),
                Token::Num(2),
                Token::Num(34),
                Token::Op('+'),
                Token::Op('-'),
                Token::Op('*'),
                Token::Op('/'),
                Token::Op('%'),
                Token::Op('('),
                Token::Op(')'),
                Token::Op('-'),
                Token::Op('^')
            ]
        );
    }
}
