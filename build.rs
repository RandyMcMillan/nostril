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
        .include("config.h")
        .file("nostril.c")
        .compile("libconfig.a");
    cc::Build::new()
        .include("cursor.h")
        .file("nostril.c")
        .compile("libcursor.a");
    cc::Build::new()
        .include("endian.h")
        .file("nostril.c")
        .compile("libendian.a");
    cc::Build::new()
        .include("hex.h")
        .file("nostril.c")
        .compile("libhex.a");
    cc::Build::new()
        .include("proof.h")
        .file("nostril.c")
        .compile("libproof.a");
    //cc::Build::new()
    //    .file("random.h")
    //    .file("nostril.c")
    //    .compile("librandom.a");
    cc::Build::new()
        .include("sha256.h")
        .file("sha256.c")
        .compile("libsha256.a");
}
       
