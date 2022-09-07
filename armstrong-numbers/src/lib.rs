pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string();
    num == num_as_string
        .chars()
        .map(|a| a.to_digit(10).unwrap())
        .map(|digit| digit.pow(num_as_string.len() as u32))
        .sum()
}
