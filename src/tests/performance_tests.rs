use crate::factorization::pollards_rho;
use crate::elliptic_curve::multiply_point;
use k256::{ProjectivePoint, Scalar};
use std::time::Instant;

#[test]
fn test_pollards_rho_performance() {
    let n = 1_000_003_003; // large composite number
    let start = Instant::now();
    let _ = pollards_rho(n);
    let duration = start.elapsed();
    println!("Pollard's Rho took: {:?}", duration);
}

#[test]
fn test_multiply_point_performance() {
    let p = ProjectivePoint::GENERATOR;
    let scalar = Scalar::from(1_000_000u64);
    let start = Instant::now();
    let _ = multiply_point(p, scalar);
    let duration = start.elapsed();
    println!("Point multiplication took: {:?}", duration);
}
