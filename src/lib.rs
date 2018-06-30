use std::collections::HashMap;

pub fn balanced<T: Into<String>>(input: T) -> bool {
    let mut stack = Vec::new();
    let closing = vec![')', '}', ']'];
    let opening = vec!['(', '{', '['];
    let map: HashMap<_, _> = closing.iter().zip(opening.iter()).collect();

    for c in input.into().chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => {
                let prev = stack.pop();
                match map.get(&c) {
                    Some(prev) => (),
                    _ => unreachable!(),
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}

pub fn word_count(line: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for word in line.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map
}
