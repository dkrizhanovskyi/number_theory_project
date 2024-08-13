use k256::{AffinePoint, ProjectivePoint, Scalar, Secp256k1};
use rand_core::OsRng;

/// This module handles elliptic curve operations over the Secp256k1 curve.
/// It includes point addition, doubling, and scalar multiplication.

/// Adds two points on the elliptic curve.
pub fn add_points(p1: ProjectivePoint, p2: ProjectivePoint) -> ProjectivePoint {
    p1 + p2
}

/// Doubles a point on the elliptic curve.
pub fn double_point(p: ProjectivePoint) -> ProjectivePoint {
    p + p
}

/// Multiplies a point on the elliptic curve by a scalar.
pub fn multiply_point(p: ProjectivePoint, scalar: Scalar) -> ProjectivePoint {
    p * scalar
}

/// Generates a random scalar and computes the corresponding point on the curve.
pub fn generate_random_point() -> (ProjectivePoint, Scalar) {
    let scalar = Scalar::generate_vartime(&mut OsRng);
    let point = ProjectivePoint::GENERATOR * scalar;
    (point, scalar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_points() {
        let p1 = ProjectivePoint::GENERATOR;
        let p2 = ProjectivePoint::GENERATOR;
        let result = add_points(p1, p2);
        assert_ne!(result, ProjectivePoint::IDENTITY);
    }

    #[test]
    fn test_double_point() {
        let p = ProjectivePoint::GENERATOR;
        let result = double_point(p);
        assert_ne!(result, ProjectivePoint::IDENTITY);
    }

    #[test]
    fn test_multiply_point() {
        let p = ProjectivePoint::GENERATOR;
        let scalar = Scalar::from(3u64);
        let result = multiply_point(p, scalar);
        assert_ne!(result, ProjectivePoint::IDENTITY);
    }

    #[test]
    fn test_generate_random_point() {
        let (point, scalar) = generate_random_point();
        assert_ne!(point, ProjectivePoint::IDENTITY);
        assert_ne!(scalar, Scalar::ZERO);
    }
}
