#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack = vec![];

    for x in inputs {
        match x {
            Value(number) => stack.push(*number),
            _ => {
                if stack.len() < 2 {
                    return None
                }

                let x2 = stack.pop().unwrap();
                let x1 = stack.pop().unwrap();

                match x {
                    Add => stack.push(x1 + x2),
                    Multiply => stack.push(x1 * x2),
                    Subtract => stack.push(x1 - x2),
                    Divide => stack.push(x1 / x2),
                    _ => return None
                }
            }, 
        }
    }

    if stack.len() != 1 {
        return None
    } 

    stack.pop()

}
