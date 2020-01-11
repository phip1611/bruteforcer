use libbruteforce::symbols;
use libbruteforce::symbols::full_alphabet;
use libbruteforce::transformation_fns::TransformationFn;
use libbruteforce::transformation_fns::MD5_HASHING;
use libbruteforce::transformation_fns::NO_HASHING;
use libbruteforce::transformation_fns::SHA1_HASHING;
use libbruteforce::transformation_fns::SHA256_HASHING;

use crate::args::CliArgs;
use core::fmt;

pub struct ProgramOptions {
    pub value_to_crack: String,
    pub min_len: usize,
    pub max_len: usize,
    pub alphabet: Box<[char]>,
    pub algo_name: String,
    pub algo: TransformationFn,
}

impl ProgramOptions {
    pub fn from(args: &CliArgs) -> ProgramOptions {
        ProgramOptions {
            value_to_crack: args.input_to_crack.as_ref().unwrap().to_string(),
            min_len: args.min_len.or(Option::from(0)).unwrap(),
            max_len: args.max_len.or(Option::from(8)).unwrap(),
            alphabet: ProgramOptions::build_alphabet(&args),
            algo_name: args
                .hashing_algo
                .as_ref()
                .map(|s| s.to_string())
                .or(Option::from("identity".to_string()))
                .unwrap(),
            algo: ProgramOptions::algo_str_to_fn(&args),
        }
    }

    fn algo_str_to_fn(args: &CliArgs) -> TransformationFn {
        let mut algo = NO_HASHING;
        if args.hashing_algo.is_some() {
            let algo_str = (args.hashing_algo).as_ref();
            let algo_str = algo_str.unwrap();
            let algo_str = String::from(algo_str).to_lowercase();
            if "md5".eq(&algo_str) {
                algo = MD5_HASHING;
            } else if "sh1".eq(&algo_str) {
                algo = SHA1_HASHING;
            } else if "sha256".eq(&algo_str) {
                algo = SHA256_HASHING;
            }
        }
        algo
    }

    fn build_alphabet(args: &CliArgs) -> Box<[char]> {
        let mut chars = vec![];

        // Highest precedence
        if args.custom_alphabet.is_some() {
            let ca = args.custom_alphabet.as_ref();
            let ca = ca.unwrap();
            ca.chars().for_each(|c| chars.push(c));
        } else {
            // map flags on build_alphabet from the library
            chars.extend_from_slice(&symbols::build_alphabet(
                args.flag_lowercase_letters,
                args.flag_uppercase_letters,
                args.flag_digits,
                args.flag_lowercase_umlauts,
                args.flag_uppercase_umlauts,
                args.flag_common_special_chars,
                args.flag_all_special_chars,
            ));

            // No options are set is equal to all are set
            if chars.len() == 0 {
                chars.extend_from_slice(&full_alphabet());
            }
        }
        chars.into_boxed_slice()
    }
}

impl fmt::Display for ProgramOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
        ProgramOptions {{
            value_to_crack: {},
            min_len: {},
            max_len: {},
            alphabet: {:?},
            algo: {},
        }}\
        ",
            self.value_to_crack, self.min_len, self.max_len, self.alphabet, self.algo_name
        )
    }
}
