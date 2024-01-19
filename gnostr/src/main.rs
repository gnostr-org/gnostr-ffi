//! //main.c
//! #include <stdint.h>
//! #include <stdio.h>
//!
//! extern int32_t double_input(int32_t input);
//!
//! int main() {
//!     int input = 4;
//!     int output = double_input(input);
//!     printf("%d * 2 = %d\n", input, output);
//!     return 0;
//! }

mod gnostr;
use crate::gnostr::double_input;

fn main() {
    let input = 4;
    let output = double_input(input);
    println!("{} * 2 = {}",input,output);
}
