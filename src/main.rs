use std::io;
use std::num::ParseIntError;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Expression {
    left: i32,
    operation: Operation,
    right: i32,
}

fn parse_experession(input: &str) -> Result<Expression, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("输入格式错误：操作数 操作符 操作数".to_string());
    }

    let left = parts[0]
        .parse::<i32>()
        .map_err(|_| "无法解析左操作数".to_string())?;
    let right = parts[2]
        .parse::<i32>()
        .map_err(|_| "无法解析右操作数".to_string())?;

    let operation = match parts[1] {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => return Err("无效的操作符".to_string()),
    };

    Ok(Expression {
        left,
        operation,
        right,
    })
}

fn calculate(experssion: &Expression) -> Result<i32, String> {
    match experssion.operation {
        Operation::Add => Ok(experssion.left + experssion.right),
        Operation::Subtract => Ok(experssion.left - experssion.right),
        Operation::Multiply => Ok(experssion.left * experssion.right),
        Operation::Divide => {
            if experssion.right == 0 {
                Err("除数不能为零".to_string())
            } else {
                Ok(experssion.left / experssion.right)
            }
        }
    }
}

fn main() {
    println!("请输入一个表达式（例如：2 + 3），按回车键计算结果。输入exit退出程序。");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        match parse_experession(input) {
            Ok(experssion) => match calculate(&experssion) {
                Ok(result) => println!("结果：{}", result),
                Err(err) => println!("计算错误：{}", err),
            },
            Err(err) => println!("解析错误：{}", err),
        }
    }
}
