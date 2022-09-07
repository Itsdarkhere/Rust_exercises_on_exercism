pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    return (1..limit)
        .filter(|val| factors.iter().any(|v| *v != 0 && val % v == 0))
        .sum()
}
