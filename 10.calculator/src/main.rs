use std::io;

#[derive(PartialEq)]
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
    let result = parse_add_sub(tokens)?;
    if !tokens.is_empty() && tokens[0] != Token::RightParen {
        Err("无效表达式".to_string())
    } else {
        Ok(result)
    }
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
    if tokens.is_empty() {
        return Err("表达式不完整".to_string());
    }

    match tokens.remove(0) {
        Token::Number(n) => Ok(n),
        Token::LeftParen => {
            let result = parse_expression(tokens)?; // 递归解析括号内的表达式
            if tokens.is_empty() {
                return Err("缺少右括号".to_string()); // 缺少右括号报错
            }
            match tokens.remove(0) {
                Token::RightParen => Ok(result), // 消耗右括号并返回结果
                _ => Err("缺少右括号".to_string()),
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
            Ok(mut tokens) => match parse_expression(&mut tokens) {
                Ok(result) => {
                    if tokens.is_empty() {
                        println!("结果：{}", result);
                    } else {
                        println!("计算错误：无效表达式");
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(input: &str) -> Result<f64, String> {
        let input = remove_whitespace(input);
        let mut tokens = tokenize(&input)?;
        parse_expression(&mut tokens)
    }

    #[test]
    fn test_basic_addition() {
        assert_eq!(eval("1 + 1").unwrap(), 2.0);
        assert_eq!(eval("3 + 4").unwrap(), 7.0);
        assert_eq!(eval("10 + 5 + 2").unwrap(), 17.0);
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(eval("5 - 2").unwrap(), 3.0);
        assert_eq!(eval("10 - 5").unwrap(), 5.0);
        assert_eq!(eval("20 - 10 - 5").unwrap(), 5.0);
    }

    #[test]
    fn test_basic_multiplication() {
        assert_eq!(eval("2 * 3").unwrap(), 6.0);
        assert_eq!(eval("4 * 5").unwrap(), 20.0);
        assert_eq!(eval("10 * 2 * 3").unwrap(), 60.0);
    }

    #[test]
    fn test_basic_division() {
        assert_eq!(eval("6 / 3").unwrap(), 2.0);
        assert_eq!(eval("20 / 4").unwrap(), 5.0);
        assert_eq!(eval("100 / 5 / 2").unwrap(), 10.0);
    }

    #[test]
    fn test_mixed_operations() {
        assert_eq!(eval("1 + 2 * 3").unwrap(), 7.0); // 乘法优先
        assert_eq!(eval("10 - 5 * 2").unwrap(), 0.0); // 乘法优先
        assert_eq!(eval("10 + 5 * 2 - 3").unwrap(), 17.0); // 混合优先级
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval("(1 + 2) * 3").unwrap(), 9.0); // 括号优先
        assert_eq!(eval("10 / (2 + 3)").unwrap(), 2.0); // 括号优先
        assert_eq!(eval("(10 - 3) * (2 + 1)").unwrap(), 21.0); // 嵌套括号
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(eval("((1 + 1) * 2) + (3 * (2 + 1))").unwrap(), 13.0); // 嵌套的括号
        assert_eq!(eval("((5))").unwrap(), 5.0); // 多层嵌套的单一表达式
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(eval("10 / 0").is_err()); // 除零错误
        assert!(eval("(5 + 5) / (2 - 2)").is_err()); // 除零错误在括号中
    }

    #[test]
    fn test_invalid_expressions() {
        assert!(eval("1 +").is_err()); // 不完整的表达式
        assert!(eval("(1 + 2").is_err()); // 缺少右括号
        assert!(eval("2 * 3)").is_err()); // 缺少左括号
        assert!(eval("abc").is_err()); // 无效字符
    }

    #[test]
    fn test_floating_point() {
        assert_eq!(eval("1.5 + 1.5").unwrap(), 3.0); // 浮点数加法
        assert_eq!(eval("2.5 * 2").unwrap(), 5.0); // 浮点数乘法
        assert_eq!(eval("5.5 / 2").unwrap(), 2.75); // 浮点数除法
    }
}
