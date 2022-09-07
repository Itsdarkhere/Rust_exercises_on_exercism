pub fn brackets_are_balanced(string: &str) -> bool {
    // Vec acting as a stack
    let mut stack = vec![];
    for x in string.chars() {
        match x {
            // Notice reverse push here
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            ']' | '}' | ')' if stack.pop() != Some(x) => return false,
            _ => (),
        }
    }
    stack.is_empty()
}
