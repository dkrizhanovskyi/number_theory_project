use crate::utils::ffi::call_c_function;

#[test]
fn test_call_c_function() {
    let input = 10;
    let result = call_c_function(input);
    assert_eq!(result, input * 2); // Например, если функция c_function умножает значение на 2
}
