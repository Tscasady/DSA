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
