use std::env;
use std::process::exit;

use libbruteforce::crack;
use libbruteforce::symbols::{build_alphabet, combinations_count, full_alphabet};
use libbruteforce::transformation_fns::identity::NO_HASHING;
use libbruteforce::transformation_fns::md5::MD5_HASHING;
use libbruteforce::transformation_fns::sha1::SHA1_HASHING;
use libbruteforce::transformation_fns::sha256::SHA256_HASHING;

fn main() {
    let alphabet = full_alphabet();

    // Typ definition necessary!
    let mut algo = NO_HASHING;
    let mut input;
    let mut max_len;

    let mut args = vec![];
    env::args().for_each(|s| args.push(s));

    if args.len() == 3 {
        println!("Using identity algorithm to crack value");
        max_len = args[1].parse::<usize>().unwrap();
        input = String::from(&args[2]);
    } else if args.len() == 4 {
        max_len = args[1].parse::<usize>().unwrap();
        input = String::from(&args[3]);
        if args[2].eq("md5") {
            algo = MD5_HASHING;
            // string representation from hashes are lowercase
            input = input.to_ascii_lowercase();
        } else if args[2].eq("sha1") {
            algo = SHA1_HASHING;
            // string representation from hashes are lowercase
            input = input.to_ascii_lowercase();
        } else if args[2].eq("sha256") {
            algo = SHA256_HASHING;
            // string representation from hashes are lowercase
            input = input.to_ascii_lowercase();
        } else {
            println!("Using identity algorithm to crack value");
        }
    } else {
        eprintln!("Usage: $ bruteforcer %length% ['md5'|'sha1'|'sha256'] %value_to_crack%");
        exit(0);
    }

    println!("Trying all {} possible combinations.", format_combinations_string(combinations_count(&alphabet, max_len as u32)));

    let _result = crack(
        input,
        alphabet,
        max_len,
        algo
    );

}

/// Makes "15252626" to "15.252.626"
fn format_combinations_string(count: usize) -> String {
    let mut formatted = String::new();
    let unformatted = format!("{}", count);
    let unformatted = unformatted.as_bytes();
    let mut x = 0;
    for i in 0..unformatted.len() {
        x += 1;
        let position = unformatted.len() - 1 - i;
        formatted.push(unformatted[position] as char);
        if x == 3 && position != 0 {
            formatted.push('.');
            x = 0;
        }
    }
    formatted.chars().rev().collect::<String>()
}