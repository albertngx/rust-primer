// e01_euclid_mcd
// The Euclidean algorithm is based on this key insight:
// the GCD of two numbers doesn't change if you replace the larger one with the remainder
// of dividing the larger by the smaller.
use std::io;

fn mcd(mut m: i64, mut n: i64) -> i64 {
    assert!(m!=0 && n!=0);
    while n!=0 {
        if n < m {
            let t = m;
            m = n;
            n = t;
        }
        n = n % m
    }
    m
}

fn input_number(idx: i64, stdin: &io::Stdin) -> i64 {
    let mut buffer = String::new();
    println!("Please, input any integer ({}).", idx);
    stdin.read_line(&mut buffer).expect("Failed to read input");
    let n: i64 = buffer.trim().parse().unwrap();
    n
}

fn main() {
    let stdin = io::stdin();
    let m = input_number(0, &stdin);
    let n = input_number(1, &stdin);
    println!("The maximum common divisor between {} and {} is: {}", m, n, mcd(m,n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcd() {
        assert_eq!(mcd(14, 15), 1);
        assert_eq!(mcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }
}