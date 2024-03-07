extern crate cc;
use std::process::Command;

fn make_all() {
    // Create a new command object
    let mut command = Command::new("make");

    // Add arguments to the command
    command.arg("all");

    // Execute the command and capture its output
    let output = command.output().expect("Failed to execute command");

    // Print the standard output
    println!(
        "Standard output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    // Check for errors (optional)
    if !output.status.success() {
        eprintln!("Error: command exited with status {}", output.status);
    }
}

fn main() {
    make_all();

    cc::Build::new().file("src/double.c").compile("libdouble.a");

    cc::Build::new()
        .include("config.h")
        .include("aes.h")
        .file("aes.c")
        .compile("libaes.a");
    cc::Build::new()
        .include("config.h")
        .include("base64.h")
        .file("base64.c")
        .compile("libbase64.a");
    cc::Build::new()
        .include("config.h")
        .file("nostril.c")
        .compile("libconfig.a");
    cc::Build::new()
        .include("config.h")
        .include("cursor.h")
        .file("nostril.c")
        .compile("libcursor.a");
    cc::Build::new()
        .include("config.h")
        .include("endian.h")
        .file("nostril.c")
        .compile("libendian.a");
    cc::Build::new()
        .include("config.h")
        .include("hex.h")
        .file("nostril.c")
        .compile("libhex.a");
    cc::Build::new()
        .include("config.h")
        .include("proof.h")
        .file("nostril.c")
        .compile("libproof.a");
    //cc::Build::new()
    //    .file("random.h")
    //    .file("nostril.c")
    //    .compile("librandom.a");
    cc::Build::new()
        .include("config.h")
        .include("sha256.h")
        .file("sha256.c")
        .compile("libsha256.a");
}
