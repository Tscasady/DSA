// use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    let left = ['(', '{', '['];
    let right = [')', '}', ']'];

    for char in s.chars() {
        match char {
            '(' | '{' | '[' => stack.push(char),
            _ => {
                let index = right.iter().position(|&x| x == char).unwrap();
                match stack.last() {
                    Some(value) => {
                        if left[index] == *value {
                            stack.pop();
                        } else {
                            stack.push(char);
                            break;
                        }
                    }
                    None => {
                        stack.push(char);
                        break;
                    }
                }
            }
        }
    }
    return stack.len() == 0;
}

struct MinStack {
    stack: Vec<(i32, i32)>
}

impl MinStack {

    fn new() -> Self {
        return MinStack {
            stack: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        if self.stack.len() == 0 {
            self.stack.push((val, val))        
        } else if self.stack.last().unwrap().1 > val {
            self.stack.push((val, val))
        } else {
            self.stack.push((val, self.stack.last().unwrap().1))
        }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().1       
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1                
    }
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();

    for token in tokens {
        match token.as_str() {
            "*" => { let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b)},
            "/" => { let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a / b)},
            "+" => { let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b)},
            "-" => { let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b)},
            _ => stack.push(token.parse::<i32>().unwrap())
        }
    }

    return stack.pop().unwrap()
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut stack = Vec::new();
    let mut result = Vec::new();

    build_solution(n, &mut stack, &mut result, 0, 0);
    
    result
}

fn build_solution(n:i32, stack: &mut Vec<&str>, result: &mut Vec<String>, open: i32, closed: i32) {
    if open == n && closed == n {
        result.push(stack.join(""));
    };

    if closed < open {
        stack.push(")");
        build_solution(n, stack, result, open, closed + 1);
        stack.pop();
    };

    if open < n {
        stack.push("(");
        build_solution(n, stack, result, open + 1, closed);
        stack.pop();
    };
}
