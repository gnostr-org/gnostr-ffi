extern crate libc;

extern "C" {
    fn double_input(input: libc::c_int) -> libc::c_int;
}
extern "C" {
    fn square_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 3;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
    let output = unsafe { square_input(input) };
    println!("{} * {} = {}", input, input, output);
}

#[cfg(test)]
mod tests {
    use super::*; // Access functions from src/lib.rs

    #[test]
    fn test_double() {
        assert_eq!(unsafe { double_input(2) }, 4);
    } // Test add function with specific inputs
    #[test]
    fn test_double_fail() {
        assert_ne!(unsafe { double_input(2) }, 5);
    } // Test add function with specific inputs
    #[test]
    fn test_square() {
        assert_eq!(unsafe { square_input(3) }, 9);
    } // Test for non-equality as well
    #[test]
    fn test_square_fail() {
        assert_ne!(unsafe { square_input(3) }, 10);
    } // Test for non-equality as well
}
