extern crate gcc;

fn main() {
    gcc::compile_library("libffiexp.a", &["src/ffiexp.c"]);
}
