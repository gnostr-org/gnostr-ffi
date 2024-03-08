extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/double.c")
        .compile("libdouble.a");
    cc::Build::new()
        .file("src/square.c")
        .compile("libsquare.a");
}
