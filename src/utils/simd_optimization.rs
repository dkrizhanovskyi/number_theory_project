use std::arch::x86_64::*;

/// Adds two vectors using SIMD operations.
pub fn simd_add(a: &[f32], b: &[f32]) -> Vec<f32> {
    let mut result = vec![0.0; a.len()];
    for i in 0..a.len() / 4 {
        unsafe {
            let va = _mm_loadu_ps(&a[i * 4] as *const f32);
            let vb = _mm_loadu_ps(&b[i * 4] as *const f32);
            let vr = _mm_add_ps(va, vb);
            _mm_storeu_ps(&mut result[i * 4] as *mut f32, vr);
        }
    }
    result
}

/// Subtracts two vectors using SIMD operations.
pub fn simd_subtract(a: &[f32], b: &[f32]) -> Vec<f32> {
    let mut result = vec![0.0; a.len()];
    for i in 0..a.len() / 4 {
        unsafe {
            let va = _mm_loadu_ps(&a[i * 4] as *const f32);
            let vb = _mm_loadu_ps(&b[i * 4] as *const f32);
            let vr = _mm_sub_ps(va, vb);
            _mm_storeu_ps(&mut result[i * 4] as *mut f32, vr);
        }
    }
    result
}
