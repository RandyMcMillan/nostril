extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/double.c")
        .compile("libdouble.a");
    cc::Build::new()
        .include("aes.h")
        .file("aes.c")
        .compile("libaes.a");
    cc::Build::new()
        .include("base64.h")
        .file("base64.c")
        .compile("libbase64.a");
    cc::Build::new()
        .include("base64.h")
        .file("base64.c")
        .compile("libbase64.a");
    cc::Build::new()
        .include("sha256.h")
        .file("sha256.c")
        .compile("libsha256.a");
}
       
