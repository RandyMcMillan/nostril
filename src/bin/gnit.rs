use bitcoin_hashes::sha256;
use bitcoin_hashes::Hash;
use clap::builder::TypedValueParser as _;
use clap::Parser;
use gnostr_bins::{blockheight, relays, weeble, wobble};
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
    let wbhw = format!("{}/{}/{}", get_weeble(), get_blockheight(), get_wobble());
    println!("{}", wbhw);

    //CAUTION: hash_of_string is different than hash.as_bytes_mut!
    //TODO: gnostr-bins: add from_str and from_bytes
    let hash_of_string_2001 = sha256::Hash::hash("2001".as_bytes());
    println!("hash_of_string_2001={}", hash_of_string_2001);
    let hash_of_weeble = sha256::Hash::hash(get_weeble().as_bytes_mut());
    println!("hash_of_weeble={}", hash_of_weeble);
    let hash_of_blockheight = sha256::Hash::hash(get_blockheight().as_bytes_mut());
    println!("hash_of_blockheight={}", hash_of_blockheight);
    let hash_of_wobble = sha256::Hash::hash(get_wobble().as_bytes_mut());
    println!("hash_of_wobble={}", hash_of_wobble);

    let hexd = hex::decode("48656c6c6f20776f726c6421");
    println!("{:?}", hexd.unwrap());
    let hello_world = b"Hello world!";
    let hello_worl = b"hello worl!_";
    println!("{:?}", hello_world);
    println!("{:?}", xor(hello_world, hello_worl));

    let args = Args::parse();
    println!("{args:?}");

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
pub fn get_wobble() -> u64 {
    wobble().unwrap() as u64
}
pub fn xor<'a>(left: &'a [u8; 12], right: &'a [u8; 12]) -> u8 {
    &left[0] ^ &right[0]
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
