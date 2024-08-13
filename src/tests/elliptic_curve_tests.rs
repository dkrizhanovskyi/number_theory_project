use crate::elliptic_curve::{add_points, double_point, generate_random_point, multiply_point};
use k256::{AffinePoint, ProjectivePoint, Scalar, Secp256k1};
use rand::rngs::OsRng; // Изменение импорта на rand::rngs::OsRng

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
