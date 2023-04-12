use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = fs::read_to_string("src/bin/day18/input.txt")?;

    let precedence = HashMap::from([('*', 1), ('+', 2)]);

    let mut sum = 0;
    for input in inputs.lines() {
        let postfix = to_postfix(input, &precedence)?;
        let result = eval_postfix(postfix);
        sum += result;
    }
    println!("The sum is: {sum}");
    Ok(())
}

#[derive(Debug)]
enum Token {
    Num(u64),
    Add,
    Mul,
}

fn eval_postfix(tokens: Vec<Token>) -> u64 {
    let mut stack = Vec::new();
    for t in tokens {
        match t {
            Token::Num(n) => stack.push(n),
            Token::Add => {
                // the expression can't be invalid, otherwise it would've failed during conversion to postfix
                let x = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(x);
            }
            Token::Mul => {
                let x = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(x);
            }
        }
    }

    stack.pop().unwrap()
}

impl TryFrom<char> for Token {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Self::Mul),
            '+' => Ok(Self::Add),
            x => char::to_digit(value, 10)
                .map(|n| Self::Num(n as u64))
                .ok_or(format!("invalid token: {x}")),
        }
    }
}

fn to_postfix(input: &str, precedence: &HashMap<char, i32>) -> Result<Vec<Token>, String> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    for c in input.chars().filter(|c| !c.is_ascii_whitespace()) {
        match c {
            '0'..='9' => postfix.push(c.try_into()?),
            '(' => stack.push(c),
            ')' => {
                while let Some(op) = stack.pop() {
                    if op != '(' {
                        postfix.push(op.try_into()?)
                    } else {
                        break;
                    }
                }
            }
            '+' | '*' => {
                while let Some(op) = stack.pop() {
                    if op == '(' || precedence[&c] > precedence[&op] {
                        stack.push(op); // put it back before breaking
                        break;
                    }
                    postfix.push(op.try_into()?);
                }
                stack.push(c);
            }
            x => panic!("Invalid expression: {x}. Parsed so far: {postfix:?}"),
        }
    }
    while let Some(x) = stack.pop() {
        if x != '(' {
            postfix.push(x.try_into()?);
        }
    }

    Ok(postfix)
}
