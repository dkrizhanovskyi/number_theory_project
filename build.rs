fn main() {
    cc::Build::new()
        .file("src/utils/c_code.c")
        .compile("c_code");
}
