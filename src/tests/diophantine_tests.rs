use crate::diophantine::{solve_linear_diophantine, gcd, extended_gcd};

#[test]
fn test_solve_linear_diophantine() {
    let result = solve_linear_diophantine(2, 3, 1);
    assert_eq!(result, Some((-1, 1)));

    let no_solution = solve_linear_diophantine(2, 4, 5);
    assert_eq!(no_solution, None);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(gcd(101, 103), 1);
}

#[test]
fn test_extended_gcd() {
    let (x, y) = extended_gcd(54, 24);
    assert_eq!(54 * x + 24 * y, 6);
}
