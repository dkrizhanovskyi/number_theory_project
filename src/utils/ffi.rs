extern "C" {
    // Пример функции на C, которую мы можем вызвать из Rust
    fn c_function(input: i32) -> i32;
}

pub fn call_c_function(x: i32) -> i32 {
    unsafe {
        c_function(x)
    }
}
