use crate::utils::simd_optimization::{simd_add, simd_subtract};

#[test]
fn test_simd_add() {
    let a = vec![1.0, 2.0, 3.0, 4.0];
    let b = vec![4.0, 3.0, 2.0, 1.0];
    let result = simd_add(&a, &b);
    assert_eq!(result, vec![5.0, 5.0, 5.0, 5.0]);
}

#[test]
fn test_simd_subtract() {
    let a = vec![5.0, 6.0, 7.0, 8.0];
    let b = vec![1.0, 2.0, 3.0, 4.0];
    let result = simd_subtract(&a, &b);
    assert_eq!(result, vec![4.0, 4.0, 4.0, 4.0]);
}
