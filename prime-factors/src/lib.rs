pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    
    match (2..=n).find(|x| n % x == 0) {
        Some(x) => {
            prime_factors.push(x);
            // Recursion, stolen from 'nebularnoise'
            prime_factors.append(&mut factors(n/x))
        },
        None => {}
    }
    prime_factors
}
