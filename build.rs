extern crate gcc;

fn main() {
    gcc::compile_library("libstb_perlin_impl.a", &["src/stb_perlin_impl.c"]);
}
