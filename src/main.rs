//! bruteforcer - A cli program written in Rust to brute force hashes using libbruteforce

/*#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]*/
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

use crate::args::analyze_args;
use crate::options::ProgramOptions;
use crate::strings::{HELP_TEXT, HELP_TEXT_SHORT};
use libbruteforce::hash_fncs::{md5_hashing, no_hashing, sha1_hashing, sha256_hashing};
use libbruteforce::symbols::{
    ALL_OTHER_SPECIAL_CHARS, COMMON_SPECIAL_CHARS, LC_UMLAUTS, UC_UMLAUTS,
};
use libbruteforce::{crack, BasicCrackParameter, CrackParameter, TargetHashInput};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;

mod args;
mod options;
mod strings;

fn main() {
    let mut args = vec![];
    env::args().for_each(|s| args.push(s));
    args.remove(0); // remove the program name
    let args = analyze_args(args);
    if args.is_none() {
        show_help(true);
        return;
    }

    // to print progress reports from libbruteforce to the terminal
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();

    let args = args.unwrap();
    if args.flag_show_help {
        show_help(false);
        return;
    }
    let ops = ProgramOptions::from(&args);

    let b_cp = BasicCrackParameter::new(ops.alphabet, ops.max_len, ops.min_len, ops.fair_mode);

    // only possible working way to abstract the hashing algorithm;
    // I didn't found a nice solution for this in libbruteforce.
    let result = match ops.algo_name.to_lowercase().as_str() {
        "md5" => crack(CrackParameter::new(
            b_cp,
            md5_hashing(TargetHashInput::HashAsStr(&ops.value_to_crack)),
        )),
        "sha1" => crack(CrackParameter::new(
            b_cp,
            sha1_hashing(TargetHashInput::HashAsStr(&ops.value_to_crack)),
        )),
        "sha256" => crack(CrackParameter::new(
            b_cp,
            sha256_hashing(TargetHashInput::HashAsStr(&ops.value_to_crack)),
        )),
        "identity" => crack(CrackParameter::new(
            b_cp,
            no_hashing(TargetHashInput::HashAsStr(&ops.value_to_crack)),
        )),
        _ => panic!("unknown variant"),
    };

    println!("Done after {:.3}s", result.duration_in_seconds());

    if let Some(solution) = result.solution() {
        println!("The password is: {}", solution);
    } else {
        println!("No solution found");
    }
}

/// Makes "15252626" to "15.252.626"
/*fn format_combinations_string(count: usize) -> String {
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
}*/

pub fn show_help(short: bool) {
    if short {
        println!("{}", HELP_TEXT_SHORT);
    } else {
        println!("{}", HELP_TEXT);
        println!();
        println!("Umlauts lower case:\n{:?}", LC_UMLAUTS);
        println!("Umlauts upper case:\n{:?}", UC_UMLAUTS);
        println!("Common special chars:\n{:?}", COMMON_SPECIAL_CHARS);
        println!("All other special chars:\n{:?}", ALL_OTHER_SPECIAL_CHARS);
    }
}
