use std::{cmp::Ordering, fmt::Binary};
use std::ops::{Add,Sub};
#[cfg(test)]


fn convert_i8_to_usize(v:i8) -> Option<usize> {
    if v < 0 || v > std::i8::MAX {
        None
    }else {
        Some(v as usize)
    }
}

fn convert_numeric_to_binary<T: Add<Output=T> + Sub<Output=T> + Ord + Binary>(n: T, size:usize) -> String{
    let bin = format!("{:01$b}", n, size);

    bin[bin.len()-4..bin.len()].to_string()
}
mod tests {
    use std::num::Wrapping;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_convert_function_with_good_value() {
       assert_eq!(convert_i8_to_usize(8), Some(8));
    }

    #[test]
    fn test_convert_function_with_negative_value() {
       assert_eq!(convert_i8_to_usize(-5), None);
    }

    #[test]
    fn test_convert_int_to_binary_positive(){
        assert_eq!(format!("{:01$b}", 10, 4), "1010");
    }

    #[test]
    fn test_convert_int_to_binary_negative(){
        let bin = format!("{:01$b}", -10, 4);
        assert_eq!(&bin[bin.len()-4..bin.len()], "0110");
    }

    #[test]
    fn test_convert_int_to_binary_positive_with_function(){
        assert_eq!(convert_numeric_to_binary(10, 4), "1010");
    }

    #[test]
    fn test_convert_int_to_binary_negative_with_function(){
        assert_eq!(convert_numeric_to_binary(-10, 4), "0110");
    }

}