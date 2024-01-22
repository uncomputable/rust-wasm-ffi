extern crate cc;

fn main() {
    cc::Build::new()
        .file("depend/foo.c")
        .file("depend/bar.c")
        .compile("depend");
}
