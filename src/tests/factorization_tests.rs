use crate::factorization::{trial_division, pollards_rho, gcd};

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
