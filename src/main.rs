use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Expression {
    left: f64,
    operation: Operation,
    right: f64,
}

fn parse_experession(input: &str) -> Result<Expression, String> {
    let mut operator_ops = None;
    let operators = ['+', '-', '*', '/'];

    for (i, ch) in input.chars().enumerate() {
        if operators.contains(&ch) {
            operator_ops = Some((i, ch));
            break;
        }
    }

    if let Some((pos, op_char)) = operator_ops {
        let left = &input[0..pos];
        let right = &input[pos + 1..];

        let left = left
            .parse::<f64>()
            .map_err(|_| "无法解析左操作数".to_string())?;
        let right = right
            .parse::<f64>()
            .map_err(|_| "无法解析右操作数".to_string())?;

        let operation = match op_char {
            '+' => Operation::Add,
            '-' => Operation::Subtract,
            '*' => Operation::Multiply,
            '/' => Operation::Divide,
            _ => return Err("无效的操作符".to_string()),
        };

        Ok(Expression {
            left,
            operation,
            right,
        })
    } else {
        Err("无效的表达式".to_string())
    }
}
fn calculate(experssion: &Expression) -> Result<f64, String> {
    match experssion.operation {
        Operation::Add => Ok(experssion.left + experssion.right),
        Operation::Subtract => Ok(experssion.left - experssion.right),
        Operation::Multiply => Ok(experssion.left * experssion.right),
        Operation::Divide => {
            if experssion.right == 0.0 {
                Err("除数不能为零".to_string())
            } else {
                Ok(experssion.left / experssion.right)
            }
        }
    }
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() {
    println!("请输入表达式 (例如: 3+4)，或输入 'exit' 退出:");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let input = remove_whitespace(input);

        match parse_experession(&input) {
            Ok(experssion) => match calculate(&experssion) {
                Ok(result) => println!("结果：{}", result),
                Err(err) => println!("计算错误：{}", err),
            },
            Err(err) => println!("解析错误：{}", err),
        }
    }
}
