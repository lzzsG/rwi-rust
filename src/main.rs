use std::io;

enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '.' => {
                let mut number_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        number_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(
                    number_str.parse().map_err(|_| "无法解析数字".to_string())?,
                ));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            _ => {
                return Err("无效字符".to_string());
            }
        }
    }
    Ok(tokens)
}

fn parse_expression(tokens: &mut Vec<Token>) -> Result<f64, String> {
    parse_add_sub(tokens)
}

fn parse_add_sub(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let mut result = parse_mutiply_divide(tokens)?;

    while let Some(token) = tokens.get(0) {
        match token {
            Token::Plus => {
                tokens.remove(0);
                result += parse_mutiply_divide(tokens)?;
            }
            Token::Minus => {
                tokens.remove(0);
                result -= parse_mutiply_divide(tokens)?;
            }
            _ => {
                break;
            }
        }
    }
    Ok(result)
}

fn parse_mutiply_divide(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let mut result = parse_primary(tokens)?;

    while let Some(token) = tokens.get(0) {
        match token {
            Token::Multiply => {
                tokens.remove(0);
                result *= parse_primary(tokens)?;
            }
            Token::Divide => {
                tokens.remove(0);
                let divisor = parse_primary(tokens)?;
                if divisor == 0.0 {
                    return Err("除数不能为0".to_string());
                }
                result /= divisor;
            }
            _ => {
                break;
            }
        }
    }
    Ok(result)
}

fn parse_primary(tokens: &mut Vec<Token>) -> Result<f64, String> {
    match tokens.remove(0) {
        Token::Number(n) => Ok(n),
        Token::LeftParen => {
            let result = parse_expression(tokens)?;
            if let Some(Token::RightParen) = tokens.get(0) {
                tokens.remove(0);
                Ok(result)
            } else {
                Err("缺少右括号".to_string())
            }
        }
        _ => Err("无效表达式".to_string()),
    }
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() {
    loop {
        println!("请输入表达式（输入q退出）：");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();
        if input == "q" {
            break;
        }

        let input = remove_whitespace(input);

        match tokenize(&input) {
            Ok(mut token) => match parse_expression(&mut token) {
                Ok(result) => {
                    println!("结果：{}", result);
                }
                Err(err) => {
                    println!("计算错误：{}", err);
                }
            },
            Err(err) => {
                println!("解析错误：{}", err);
            }
        }
    }
}
