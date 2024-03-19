use std::env;
use std::fs::File;
use sha2::{Sha256, Digest};
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid amount of arguments!");
        println!("How to use: cargo run <sha256 hash>");
        exit(1);
    }

    let wanted_hash = &args[1];
    let passwd_file = "src/passwd_list.txt";
    let mut attempts = 1;

    println!("Attempting to crack: {}!\n", wanted_hash);

    let passwd_list = File::open(passwd_file).unwrap();
    let reader = BufReader::new(passwd_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let passwd = line.trim().to_owned().into_bytes();
        let passwd_hash = format!("{:x}", Sha256::digest(&passwd));

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&passwd).unwrap(), passwd_hash);

        if &passwd_hash == wanted_hash {
            println!("\nPassword hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&passwd).unwrap(), passwd_hash);
            exit(0);
        }

        attempts += 1;
    }

    println!("Password hash not found!");

}

