// TODO: add Doc-test.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i128),
    FNum(f64),
    Op(char),
    Ident(String),
}

/// Cut out sequense of num_char as `String` from input `chars: &[char]`.
/// Increment index and return as a member of tupple.
fn tok_get_num(chars: &[char], index: usize) -> (String, usize) {
    let mut i = index;
    if i < chars.len() {
        match chars[i] {
            '-' | '0'..='9' => {
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
            _ => (String::from('0'), i),
        }
    } else {
        (String::from('0'), i)
    }
}

/// Eat integer numbers from input array.
/// Return `Token::Num()` with `Result<,Err(String)>`.
/// Increment index and return as a member of tupple.
fn tok_num_int(chars: &[char], index: usize) -> (Result<Token, String>, usize) {
    let mut i = index;
    let radix: u32;
    let mut mantissa = String::from("0");
    let mut err_str = String::from("0");

    if i < chars.len() {
        match chars[i] {
            'x' | 'X' => {
                radix = 16;
                i += 1;
                err_str.push(chars[i]);
            }
            'b' | 'B' => {
                radix = 2;
                i += 1;
                err_str.push(chars[i]);
            }
            '0'..='7' => {
                radix = 8;
            }
            _ => {
                return (Ok(Token::Num(0)), i);
            }
        }
    } else {
        return (Ok(Token::Num(0)), i);
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
        Ok(int) => (Ok(Token::Num(int)), i),
        Err(e) => (Err(format!("Error: Integer format: {} {}", e, err_str)), i),
    }
}

/// Eat numbers from input array.
/// Forwared to `tok_num_int()` when interger, i.e. decimal, hexdecimal, octal or binary.
/// Return `Token::Num()` or `Token::FNum()` with `Result<,Err(String)>`.
/// Increment index and return as a member of tupple.
fn tok_num(chars: &[char], index: usize) -> (Result<Token, String>, usize) {
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
                    return (Ok(Token::Num(0)), i);
                }
            }
        } else {
            return (Ok(Token::Num(0)), i + 1);
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
                return (Ok(Token::Num(int)), i);
            }
            Err(e) => {
                return (Err(format!("Error: Integer format: {} {}", e, mantissa)), i);
            }
        }
    }
    if has_exponent {
        mantissa.push('e');
        mantissa.push_str(&exponent);
    }
    match mantissa.parse::<f64>() {
        Ok(float) => (Ok(Token::FNum(float)), i),
        Err(e) => (Err(format!("Error: Float format: {} {}", e, mantissa)), i),
    }
}

/// Input: `String`
/// Output: `Result<Vec<Token>, String>`
///
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
// TODO: handle vars/functions.
// TODO: support SI postifx(k/M/G/T/m/u/n/p)
pub fn lexer(s: String) -> Result<Vec<Token>, String> {
    let mut ret = Vec::new();

    let chars: Vec<char> = s.chars().collect();
    let mut i: usize = 0;
    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                // `Num` or `FNum` begin from '0'..='9'.
                let (tk, b) = tok_num(&chars, i);
                i = b;
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
                // operators
                ret.push(Token::Op(chars[i]));
                i += 1;
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

    #[test]
    fn test_tok_get_num() {
        assert_eq!(
            tok_get_num(&("0".chars().collect::<Vec<char>>()), 0),
            ("0".to_string(), 1)
        );
        assert_eq!(
            tok_get_num(&("1".chars().collect::<Vec<char>>()), 0),
            ("1".to_string(), 1)
        );
        assert_eq!(
            tok_get_num(&("34".chars().collect::<Vec<char>>()), 0),
            ("34".to_string(), 2)
        );
        assert_eq!(
            tok_get_num(&("56a".chars().collect::<Vec<char>>()), 0),
            ("56".to_string(), 2)
        );
        assert_eq!(
            tok_get_num(&("".chars().collect::<Vec<char>>()), 0),
            ("0".to_string(), 0)
        );
        assert_eq!(
            tok_get_num(&("a".chars().collect::<Vec<char>>()), 0),
            ("0".to_string(), 0)
        );
    }
    #[test]
    fn test_tok_num_int() {
        assert_eq!(
            tok_num_int(&("0x1".chars().collect::<Vec<char>>()), 1),
            (Ok(Token::Num(1)), 3)
        );
        assert_eq!(
            tok_num_int(&("0xa".chars().collect::<Vec<char>>()), 1),
            (Ok(Token::Num(10)), 3)
        );
        assert_eq!(
            tok_num_int(&("0x10".chars().collect::<Vec<char>>()), 1),
            (Ok(Token::Num(16)), 4)
        );
        assert_eq!(
            tok_num_int(&("0b10".chars().collect::<Vec<char>>()), 1),
            (Ok(Token::Num(2)), 4)
        );
        assert_eq!(
            tok_num_int(&("0b1_0".chars().collect::<Vec<char>>()), 1),
            (Ok(Token::Num(2)), 5)
        );
        assert_eq!(
            tok_num_int(&("010".chars().collect::<Vec<char>>()), 1),
            (Ok(Token::Num(8)), 3)
        );
    }
    #[test]
    fn test_tok_num() {
        assert_eq!(
            tok_num(&("0x1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(1)), 3)
        );
        assert_eq!(
            tok_num(&("0xa".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(10)), 3)
        );
        assert_eq!(
            tok_num(&("0x10".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(16)), 4)
        );
        assert_eq!(
            tok_num(&("0b10".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(2)), 4)
        );
        assert_eq!(
            tok_num(&("0b1_0".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(2)), 5)
        );
        assert_eq!(
            tok_num(&("010".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(8)), 3)
        );
        assert_eq!(
            tok_num(&("10".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(10)), 2)
        );
        assert_eq!(
            tok_num(&("10.1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(10.1)), 4)
        );
        assert_eq!(
            tok_num(&("10e1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(100.0)), 4)
        );
        assert_eq!(
            tok_num(&("10e1+".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(100.0)), 4)
        );
        assert_eq!(
            tok_num(&("1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(1)), 1)
        );
        assert_eq!(
            tok_num(&("1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(1)), 1)
        );
        assert_eq!(
            tok_num(&("0".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(0)), 1)
        );
        assert_eq!(
            tok_num(&("10".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(10)), 2)
        );
        assert_eq!(
            tok_num(&("1.1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(1.1)), 3)
        );
        assert_eq!(
            tok_num(&("0.1".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(0.1)), 3)
        );
        assert_eq!(
            tok_num(&("1.1E2".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(110.0)), 5)
        );
        assert_eq!(
            tok_num(&("1.1E-2".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(0.011)), 6)
        );
        assert_eq!(
            tok_num(&("100_000".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(100000)), 7)
        );
        assert_eq!(
            tok_num(&("0xa".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(10)), 3)
        );
        assert_eq!(
            tok_num(&("011".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(9)), 3)
        );
        assert_eq!(
            tok_num(&("0b11".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(3)), 4)
        );
        assert_eq!(
            tok_num(&("1e3".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::FNum(1000.0)), 3)
        );
        assert_eq!(
            tok_num(&("9223372036854775807".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(9223372036854775807)), 19)
        );
        assert_eq!(
            tok_num(&("18446744073709551615".chars().collect::<Vec<char>>()), 0),
            (Ok(Token::Num(18446744073709551615)), 20)
        );
    }

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
