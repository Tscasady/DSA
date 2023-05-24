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
