use std::process::Command;

fn main() {
    Command::new("cargo").args(["update", "nostr-types"]);
    Command::new("brew").args(["install", "openssl@1.1", "||", "true"]);
    Command::new("apt").args(["install", "openssl", "||", "true"]);
    Command::new("apk").args(["add", "openssl-dev", "||", "true"]);
}
