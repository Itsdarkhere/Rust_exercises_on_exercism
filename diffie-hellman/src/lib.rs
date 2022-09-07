use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

// optimized modular_pow from: https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0
    }

    // using u64 for intermediate calculation can
    // result in a b == 0 value along the way
    // once b = 0, there's no escaping 0
    // using u128 avoids that
    let mut result = 1u128;
    let mut e = exponent as u128;
    let mut b = base as u128;
    let m = modulus as u128;
    assert_ne!((m - 1).checked_mul(m - 1), None);
    b = b % m;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        e = e >> 1;
        b = (b * b) % m;
    }

    result as u64 % modulus

}
