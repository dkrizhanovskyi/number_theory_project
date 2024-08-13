/// This module provides methods for solving Diophantine equations, which are
/// equations that require integer solutions. The most well-known examples
/// include linear Diophantine equations and Pell's equation.

/// Solves a linear Diophantine equation of the form ax + by = c.
/// Returns a solution (x, y) if one exists, or None if there is no solution.
pub fn solve_linear_diophantine(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let g = gcd(a, b);
    if c % g != 0 {
        return None; // No solution exists
    }

    let (x0, y0) = extended_gcd(a, b);
    let x = x0 * (c / g);
    let y = y0 * (c / g);
    Some((x, y))
}

/// Solves Pell's equation of the form x^2 - dy^2 = 1.
/// Returns a solution (x, y) if one exists, or None if there is no solution.
pub fn solve_pell_equation(d: i64) -> Option<(i64, i64)> {
    // Placeholder for a more complex algorithm such as the Chakravala method
    // or continued fractions method for solving Pell's equation.
    None
}

/// Computes the greatest common divisor of a and b using the Euclidean algorithm.
pub fn gcd(a: i64, b: i64) -> i64 {  // Сделать функцию публичной
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// Computes the extended GCD of a and b.
/// Returns a tuple (x, y) such that ax + by = gcd(a, b).
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        (1, 0)
    } else {
        let (x, y) = extended_gcd(b, a % b);
        (y, x - (a / b) * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
