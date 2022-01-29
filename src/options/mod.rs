use crate::args::CliArgs;
use libbruteforce::symbols;

#[derive(Debug)]
pub struct ProgramOptions {
    pub value_to_crack: String,
    pub min_len: u32,
    pub max_len: u32,
    pub alphabet: Box<[char]>,
    pub algo_name: String,
    pub fair_mode: bool,
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
            fair_mode: args.flag_fair_mode,
        }
    }

    fn build_alphabet(args: &CliArgs) -> Box<[char]> {
        if args.custom_alphabet.is_some() {
            let mut chars = vec![];
            let ca = args.custom_alphabet.as_ref();
            let ca = ca.unwrap();
            ca.chars().for_each(|c| chars.push(c));
            chars.into_boxed_slice()
        } else {
            let mut builder = symbols::Builder::new();
            if args.flag_lowercase_letters {
                builder = builder.with_lc_letters();
            }
            if args.flag_uppercase_letters {
                builder = builder.with_uc_letters();
            }
            if args.flag_digits {
                builder = builder.with_digits();
            }
            if args.flag_lowercase_umlauts {
                builder = builder.with_lc_umlauts();
            }
            if args.flag_uppercase_umlauts {
                builder = builder.with_uc_umlauts();
            }
            if args.flag_common_special_chars {
                builder = builder.with_common_special_chars();
            }
            if args.flag_all_special_chars {
                builder = builder.with_all_special_chars();
            }

            // No options are set is equal to all are set
            if builder.is_empty() {
                builder = builder.full();
            }

            builder.build()
        }
    }
}
