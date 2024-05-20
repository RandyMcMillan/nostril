use std::io;
use std::process::Command;
use std::{env, fs};

use std::env::args;

use include_dir::{include_dir, Dir};
//use std::path::Path;
use markdown::to_html;

//static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

fn main() -> io::Result<()> {
    //capture git-nostril --sec <private_key>
    let args_vec: Vec<String> = env::args().collect();
    let mut app: &String = &("").to_string();
    let mut sec: &String = &("").to_string();
    let mut private_key: &String = &("").to_string();
    if args_vec.len() > 2 {
    app = &args_vec[0];
    sec = &args_vec[1];
    }
    //println!("app={}", &app);
    //println!("sec={}", &sec);
    if args_vec.len() >= 3 {
    private_key = &args_vec[2];
    }
    //println!("private_key={}", &private_key);

    //skip git-nostril --sec <private_key>
    //and capture everything else
    let args: Vec<String> = env::args().skip(3).collect();
    //println!("args={:?}", &args);
    let which_nostril = Command::new("which")
        .arg("nostril")
        .output()
        .expect("failed to execute process");
    let nostril = String::from_utf8(which_nostril.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    let event = Command::new("nostril")
        .arg(&sec)
        .arg(&private_key)
        .args(&args)
        .output()
        .expect("failed to execute process");

    let nostril_event = String::from_utf8(event.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    println!("{}", nostril_event);
    Ok(())
}
