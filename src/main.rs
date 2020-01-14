use std::env;
use std::time::Instant;

use libbruteforce::symbols::{
    combinations_count, ALL_OTHER_SPECIAL_CHARS, COMMON_SPECIAL_CHARS, LC_UMLAUTS, UC_UMLAUTS,
};
use libbruteforce::{crack, CrackParameter};

use crate::args::analyze_args;
use crate::options::ProgramOptions;
use crate::strings::{HELP_TEXT, HELP_TEXT_SHORT};

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
    let args = args.unwrap();
    if args.flag_show_help {
        show_help(false);
        return;
    }
    let ops = ProgramOptions::from(&args);

    let cp = CrackParameter::new(
        ops.value_to_crack,
        ops.alphabet,
        ops.max_len,
        ops.min_len,
        ops.algo,
        ops.fair_mode,
    );

    let result = crack(cp);

    println!("done after {}s", result.seconds_as_fraction);

    if result.is_success() {
        println!("The password is: {}", result.solution.unwrap());
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
