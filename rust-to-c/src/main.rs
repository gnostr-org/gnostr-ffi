extern crate libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}
extern {
    fn square_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
    let output = unsafe { square_input(input) };
    println!("{} * {} = {}", input,input,  output);
}
