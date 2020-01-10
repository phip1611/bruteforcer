use std::env;
use std::process::exit;

mod strings;
mod args;
mod options;

use libbruteforce::crack;
use libbruteforce::symbols::{build_alphabet, combinations_count, full_alphabet, UC_UMLAUTS, LC_UMLAUTS, COMMON_SPECIAL_CHARS, ALL_OTHER_SPECIAL_CHARS};
use libbruteforce::transformation_fns::identity::NO_HASHING;
use libbruteforce::transformation_fns::md5::MD5_HASHING;
use libbruteforce::transformation_fns::sha1::SHA1_HASHING;
use libbruteforce::transformation_fns::sha256::SHA256_HASHING;
use crate::args::analyze_args;
use crate::strings::{HELP_TEXT, HELP_TEXT_SHORT};

fn main() {
    let mut args = vec![];
    env::args().for_each(|s| args.push(s));
    args.remove(0); // remove the program name
    let args = analyze_args(args);

    if args.is_none() {
        show_help(true);
        return;
    }
    let args = args.unwrap();
    if args.flag_show_help {
        show_help(false);
        return;
    }

/*
    let _result = crack(
        input,
        alphabet,
        max_len,
        algo
    );*/

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

pub fn show_help(short: bool) {
    if short {
        println!("{}", HELP_TEXT_SHORT);
    } else {
        println!("{}", HELP_TEXT);
        println!("Umlauts lower case:\n{:?}", LC_UMLAUTS);
        println!("Umlauts upper case:\n{:?}", UC_UMLAUTS);
        println!("Common special chars:\n{:?}", COMMON_SPECIAL_CHARS);
        println!("All other special chars:\n{:?}", ALL_OTHER_SPECIAL_CHARS);
    }
}