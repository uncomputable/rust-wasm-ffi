extern crate cc;

fn main() {
    cc::Build::new()
        .file("depend/foo.c")
        .compile("foo");
}
