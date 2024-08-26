fn main() {
    cc::Build::new()
        .file("lib/spam.c")
        .compile("spam");
}
