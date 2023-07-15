#![allow(unused)]
#![allow(dead_code)]
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
//use std::time::SystemTime;
use std::io::{Result};
use std::env;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::convert::TryInto;
use std::any::type_name;
//use std::mem::size_of;
use std::{io, thread};
use argparse::{ArgumentParser,Store};
use gitminer::Gitminer;
use git2::*;
use crypto::sha2::Sha256;

mod worker;
mod gitminer;
mod repo;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

//fn convert_to_u32(v: usize) -> Option<i8> {
//    if v > (std::i8::MAX as i32).try_into().unwrap() {
//        None
//    } else {
//        Some(v as i8)
//    }
//}

//fn get_current_working_dir() -> std::io::Result<PathBuf> {
//    env::current_dir()
//}

fn main() -> io::Result<()> {

    let start = time::get_time();
    let system_time = SystemTime::now();
        //.duration_since(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Utc> = system_time.into();
    println!("{}", datetime.format("%d/%m/%Y %T"));
    let state = repo::state();

    let count = thread::available_parallelism()?.get();
    assert!(count >= 1_usize);
    //println!("{}={}", type_of(count), (count as i32));
    //println!("{}={}", type_of(count), (count as i64));
    let mut sha256 = Sha256::new();
    //sha256.input_str(count);
    //let ip_address_hash: String = format!("{:X}", sha256.finalize());
   let now = SystemTime::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
    match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("{}", elapsed.as_secs());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {e:?}");
       }
   }
    let path = env::current_dir()?;
        println!("The current directory is {}", path.display());
        //Ok(());
    let mut opts = gitminer::Options{
        threads: count.try_into().unwrap(),
        target:  "gnostr".to_string(),
        //gnostr:##:nonce
        //part of the gnostr protocol
        //src/worker.rs adds the nonce
        message: "gnostr".to_string(),
        //message: count.to_string(),
        //repo:    ".".to_string(),
        repo:    path.as_path().display().to_string(),
        timestamp: time::now(),
            //.duration_since(SystemTime::UNIX_EPOCH)
    };

    parse_args_or_exit(&mut opts);

    let mut miner = match Gitminer::new(opts) {
        Ok(m)  => m,
        Err(e) => { panic!("Failed to start git miner: {}", e); }
    };

    let hash = match miner.mine() {
        Ok(s)  => s,
        Err(e) => { panic!("Failed to generate commit: {}", e); }
    };

    let duration = time::get_time() - start;
    println!("Success! Generated commit {} in {} seconds", hash, duration.num_seconds());
    Ok(())
}

fn parse_args_or_exit(opts: &mut gitminer::Options) {
    let mut ap = ArgumentParser::new();
    ap.set_description("Generate git commit sha with a custom prefix");
    ap.stop_on_first_argument(false);

    //ap.refer(&mut opts.repo)
    //    //.add_argument("repository-path", Store, "Path to your git repository (required)");
    //    .add_argument("repository-path", Store, "Path to your git repository");
    //    //.required();
    ap.refer(&mut opts.repo).add_argument("repository-path", Store, "Path to your git repository");

    ap.refer(&mut opts.target)
        .add_option(&["-p", "--prefix"], Store, "Desired commit prefix (required)");
        //.required();

    ap.refer(&mut opts.threads)
        .add_option(&["-t", "--threads"], Store, "Number of worker threads to use (default 8)");

    ap.refer(&mut opts.message)
        .add_option(&["-m", "--message"], Store, "Commit message to use (required)");
        //.required();

    //ap.refer(&mut opts.timestamp)
    //    .add_option(&["--timestamp"], Store, "Commit timestamp to use (default now)");

    ap.parse_args_or_exit();
}
