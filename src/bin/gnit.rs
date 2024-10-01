use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;
use clap::builder::TypedValueParser as _;
use clap::Parser;
use gnostr_bins::{blockhash, blockheight, relays, weeble, wobble};
use std::error::Error;
use std::process::exit;
use zerocopy::AsBytes;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(term_width = 0)] // Just to make testing across clap features easier
struct Args {
    /// Implicitly using `std::str::FromStr`
    #[arg(short = 'O')]
    optimization: Option<usize>,

    /// Allow invalid UTF-8 paths
    #[arg(short = 'I', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    include: Option<std::path::PathBuf>,

    /// Handle IP addresses
    #[arg(long)]
    bind: Option<std::net::IpAddr>,

    /// Allow human-readable durations
    #[arg(long)]
    sleep: Option<humantime::Duration>,

    /// Hand-written parser for tuples
    #[arg(short = 'D', value_parser = parse_key_val::<String, i32>)]
    defines: Vec<(String, i32)>,

    /// Support for discrete numbers
    #[arg(
        long,
        default_value_t = 22,
        value_parser = clap::builder::PossibleValuesParser::new(["22", "80"])
            .map(|s| s.parse::<usize>().unwrap()),
    )]
    port: usize,

    /// Support enums from a foreign crate that don't implement `ValueEnum`
    #[arg(
        long,
        default_value_t = foreign_crate::LogLevel::Info,
        value_parser = clap::builder::PossibleValuesParser::new(["trace", "debug", "info", "warn", "error"])
            .map(|s| s.parse::<foreign_crate::LogLevel>().unwrap()),
    )]
    log_level: foreign_crate::LogLevel,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

mod foreign_crate {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub(crate) enum LogLevel {
        Trace,
        Debug,
        Info,
        Warn,
        Error,
    }

    impl std::fmt::Display for LogLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Self::Trace => "trace",
                Self::Debug => "debug",
                Self::Info => "info",
                Self::Warn => "warn",
                Self::Error => "error",
            };
            s.fmt(f)
        }
    }
    impl std::str::FromStr for LogLevel {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "trace" => Ok(Self::Trace),
                "debug" => Ok(Self::Debug),
                "info" => Ok(Self::Info),
                "warn" => Ok(Self::Warn),
                "error" => Ok(Self::Error),
                _ => Err(format!("Unknown log level: {s}")),
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let bhashwbhw = format!(
        "{}/{}/{}/{}",
        get_blockhash(),
        get_weeble(),
        get_blockheight(),
        get_wobble()
    );
    //println!("{}", bhashwbhw);

    //CAUTION: hash_of_string is different than hash.as_bytes_mut!
    //TODO: gnostr-bins: add from_str and from_bytes
    let hash_of_string_2000 = sha256::Hash::hash("2000".as_bytes());
    //println!("hash_of_string_2000={}", hash_of_string_2000);
    //println!("hash_of_string_2000={:x}", hash_of_string_2000);

    //println!("get_weeble()={}", get_weeble());
    //println!("get_weeble_string()={}", get_weeble_string());

    let hash_of_weeble = sha256::Hash::hash(get_weeble().as_bytes_mut());
    //println!("hash_of_weeble={}", hash_of_weeble);

    let hash_of_weeble_string = sha256::Hash::hash(get_weeble_string().as_bytes());
    //println!("hash_of_weeble_string={}", hash_of_weeble_string);

    let hash_of_blockheight = sha256::Hash::hash(get_blockheight().as_bytes_mut());
    //println!("hash_of_blockheight={}", hash_of_blockheight);

    let hash_of_wobble = sha256::Hash::hash(get_wobble().as_bytes_mut());
    //println!("hash_of_wobble={}", hash_of_wobble);
    //println!("{:?}", xor(hash_of_wobble, b"000000000000"));

    //let res = xor(b"100f00a00000a000000000fff0000000", b"0000f00000a00000000fff0000000001");
    //println!("\nres={:?}",res);
    //let res = xor(b"900f00900090a0900009000900090000", b"1000a0000000000010000fff00000001");
    //println!("\nres={:?}",res);
    //let res = xor(b"9000A0900090f0900009000900090099", b"10000f00000A00001000f000A00A0001");
    //println!("\nres={:?}",res);

    //let res = xor(b"ffffffffffffffffffffffffffffffff", b"10000f00000A00001000f000A00A0001");
    //println!("\nres={:?}",res);

    //println!("\nxor:{:?}", xor(b"1000000000000000000000000000000f", b"9000000000000000000000000000000f"));
    //println!("\nxor:{:?}", xor(b"010000000000000000000000000000f0", b"090000000000000000000000000000f0"));
    //println!("\nxor:{:?}", xor(b"00100000000000000000000000000f00", b"00900000000000000000000000000f00"));
    //println!("\nxor:{:?}", xor(b"0001000000000000000000000000f000", b"0009000000000000000000000000f000"));
    //println!("\nxor:{:?}", xor(b"000010000000000000000000000f0000", b"000090000000000000000000000f0000"));
    //println!("\nxor:{:?}", xor(b"00000100000000000000000000f00000", b"00000900000000000000000000f00000"));
    //println!("\nxor:{:?}", xor(b"0000001000000000000000000f000000", b"0000009000000000000000000f000000"));
    //println!("\nxor:{:?}", xor(b"000000010000000000000000f0000000", b"000000090000000000000000f0000000"));
    //println!("\nxor:{:?}", xor(b"00000000100000000000000f00000000", b"00000000900000000000000f00000000"));
    //println!("\nxor:{:?}", xor(b"0000000001000000000000f000000000", b"0000000009000000000000f000000000"));
    //println!("\nxor:{:?}", xor(b"000000000010000000000f0000000000", b"000000000090000000000f0000000000"));
    //println!("\nxor:{:?}", xor(b"00000000000100000000f00000000000", b"00000000000900000000f00000000000"));
    //println!("\nxor:{:?}", xor(b"0000000000001000000f000000000000", b"0000000000009000000f000000000000"));
    //println!("\nxor:{:?}", xor(b"000000000000010000f0000000000000", b"000000000000090000f0000000000000"));
    //println!("\nxor:{:?}", xor(b"00000000000000100f00000000000000", b"00000000000000900f00000000000000"));
    //println!("\nxor:{:?}", xor(b"0000000000000001f000000000000000", b"0000000000000009f000000000000000"));
    //println!("\nxor:{:?}", xor(b"000000000000000f1000000000000000", b"000000000000000f9000000000000000"));
    //println!("\nxor:{:?}", xor(b"00000000000000f00100000000000000", b"00000000000000f00900000000000000"));
    //println!("\nxor:{:?}", xor(b"0000000000000f000010000000000000", b"0000000000000f000090000000000000"));
    //println!("\nxor:{:?}", xor(b"000000000000f0000001000000000000", b"000000000000f0000009000000000000"));
    //println!("\nxor:{:?}", xor(b"00000000000f00000000100000000000", b"00000000000f00000000900000000000"));
    //println!("\nxor:{:?}", xor(b"0000000000f000000000010000000000", b"0000000000f000000000090000000000"));
    //println!("\nxor:{:?}", xor(b"000000000f0000000000001000000000", b"000000000f0000000000009000000000"));
    //println!("\nxor:{:?}", xor(b"00000000f00000000000000100000000", b"00000000f00000000000000900000000"));
    //println!("\nxor:{:?}", xor(b"0000000f000000000000000010000000", b"0000000f000000000000000090000000"));
    //println!("\nxor:{:?}", xor(b"000000f0000000000000000001000000", b"000000f0000000000000000009000000"));
    //println!("\nxor:{:?}", xor(b"00000f00000000000000000000100000", b"00000f00000000000000000000900000"));
    //println!("\nxor:{:?}", xor(b"0000f000000000000000000000010000", b"0000f000000000000000000000090000"));
    //println!("\nxor:{:?}", xor(b"000f0000000000000000000000001000", b"000f0000000000000000000000009000"));
    //println!("\nxor:{:?}", xor(b"00f00000000000000000000000000100", b"00f00000000000000000000000000900"));
    //println!("\nxor:{:?}", xor(b"0f000000000000000000000000000010", b"0f000000000000000000000000000090"));
    //println!("\nxor:{:?}", xor(b"f0000000000000000000000000000001", b"f0000000000000000000000000000009"));

    let mut sha256_0 = hex::decode(b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    //for byte in &sha256_0 {
    //for byte in &sha256_0 {
    for byte in &sha256_0.as_mut() {
        //if let Ok(byte) = &sha256_0 {
        //print!("\n{:?}", byte);
        //}
    }
    //println!("sha256_0.unwrap()=\n{:?}", sha256_0.unwrap());

    //for byte in &sha256_0 {
      //  print!("{:?}",sha256_0.clone().unwrap().pop())
        //}

    let hexd = hex::decode(b"48656c6c6f20776f726c6421");
    //println!("hexd.unwrap()=\n{:?}", hexd.unwrap());
    let hello_world = b"Hello world!00000000000000000000";
    let hello_worl = b"hello worl!_00000000000000000000";
    //println!("hello_world=\n{:?}", hello_world);
    //println!("xored\n{:?}", xor(hello_world, hello_worl));
    let _ = xor(hello_world, hello_worl);

    let args = Args::parse();
    //println!("{args:?}");

    exit(0)
}

pub fn get_relays_list() -> String {
    relays().unwrap()
}
pub fn get_weeble() -> u64 {
    weeble().unwrap() as u64
}
pub fn get_blockheight() -> u64 {
    blockheight().unwrap() as u64
}
pub fn get_blockhash() -> String {
    blockhash().unwrap()
}
pub fn get_wobble() -> u64 {
    wobble().unwrap() as u64
}
pub fn get_weeble_string() -> String {
    format!("{}", weeble().unwrap())
}
pub fn get_blockheight_string() -> String {
    format!("{}", blockheight().unwrap())
}
pub fn get_wobble_string() -> String {
    format!("{}", wobble().unwrap())
}
pub fn xor<'a>(left: &'a [u8; 32], right: &'a [u8; 32]) -> Vec<&'a u8> {
    let mut result = Vec::new();
    let mut left_count = left.len();
    //println!("{}", left_count);
    let right_count = right.len();
    //println!("{}", right_count);
    for byte in left {
        result.push(byte);
    }
    for byte in right {
        //result.push(byte);
    }
    for byte in &result {
        print!("{}", byte);
        if left_count > 0 && left_count < right_count {
            let _ = (result[left_count] ^ right[left_count]);
            left_count -= 1;
        }
    }
    result
    //(left[0] ^ right[0]).into()
}
pub fn div(left: usize, right: usize) -> usize {
    left / right
}
pub fn modulus(left: usize, right: usize) -> usize {
    left % right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weeble_mod_blockheight() {
        let result = get_weeble() % get_blockheight();
        assert_eq!(result % 1 as u64, 0);
    }
    #[test]
    fn blockheight_mod_weeble() {
        let result = get_blockheight() % get_weeble();
        assert_eq!(result % 1 as u64, 0);
    }
}
