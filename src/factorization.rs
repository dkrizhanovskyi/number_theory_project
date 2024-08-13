use num::integer::Roots;
use rayon::prelude::*;

/// This module provides algorithms for integer factorization, including
/// trial division, Pollard's Rho algorithm, and the quadratic sieve.

/// Performs trial division to find a factor of `n`.
/// Uses parallel iteration to speed up the process.
pub fn trial_division(n: u64) -> Option<u64> {
    if n <= 1 {
        return None;
    }

    // Parallel search for a divisor
    (2..=n.sqrt()).into_par_iter().find_any(|&i| n % i == 0)
}

/// Pollard's Rho algorithm for integer factorization.
/// Returns a non-trivial factor of `n`, or None if no factor is found.
pub fn pollards_rho(n: u64) -> Option<u64> {
    if n <= 1 {
        return None;
    }
    if n % 2 == 0 {
        return Some(2);
    }

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;
    let f = |x: u64| (x * x + 1) % n;

    while d == 1 {
        x = f(x);
        y = f(f(y));
        d = gcd((x as i64 - y as i64).abs() as u64, n);
    }

    if d == n {
        None
    } else {
        Some(d)
    }
}

/// Quadratic sieve algorithm placeholder.
/// This is a placeholder for a more complex algorithm.
pub fn quadratic_sieve(n: u64) -> Option<u64> {
    // Placeholder for a more advanced factorization algorithm
    None
}

/// Computes the greatest common divisor of a and b using the Euclidean algorithm.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_division() {
        assert_eq!(trial_division(15), Some(3));
        assert_eq!(trial_division(17), None); // 17 is prime
    }

    #[test]
    fn test_pollards_rho() {
        assert_eq!(pollards_rho(91), Some(7));
        assert_eq!(pollards_rho(17), None); // 17 is prime
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(101, 103), 1);
    }
}
