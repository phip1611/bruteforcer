use libbruteforce::transformation_fns::identity::NO_HASHING;

pub fn analyze_args(args: Vec<String>) -> Option<CliArgs> {
    if args.is_empty() {
        return None;
    }

    let mut cli_args = CliArgs::new();
    let mut i = 0;
    for arg in args {
        if "-L".eq(&arg) {
            cli_args.flag_uppercase_letters = true;
        } else if "-l".eq(&arg) {
            cli_args.flag_lowercase_letters = true;
        } else if "-d".eq(&arg) {
            cli_args.flag_digits = true;
        } else if "-U".eq(&arg) {
            cli_args.flag_uppercase_umlauts = true;
        } else if "-u".eq(&arg) {
            cli_args.flag_lowercase_umlauts = true;
        } else if "-S".eq(&arg) {
            cli_args.flag_all_special_chars = true;
            cli_args.flag_common_special_chars = true;
        } else if "-s".eq(&arg) {
            cli_args.flag_common_special_chars = true;
        } else if "-h".eq(&arg) {
            cli_args.flag_show_help = true;
        } else if i == 0 {
            i += 1;
            cli_args.input_to_crack = Some(arg);
        } else if i == 1 {
            i += 1;
            cli_args.input_to_crack = Some(arg);
        } else if i == 2 {
            i += 1;
            cli_args.max_len = Some(arg.parse::<usize>().expect("Must be a number!"));
        } else if i == 3 {
            i += 1;
            cli_args.min_len = Some(arg.parse::<usize>().expect("Must be a number!"));
        }
    }

    Some(cli_args)
}

/// Struct that represents every possible value/flag you can pass to the program to
/// further analyze what's to do in a next step.
pub struct CliArgs {
    pub input_to_crack: Option<String>,
    pub hashing_algo: Option<String>,
    pub max_len: Option<usize>,
    pub min_len: Option<usize>,
    pub flag_show_help: bool,
    pub flag_lowercase_letters: bool,
    pub flag_uppercase_letters: bool,
    pub flag_lowercase_umlauts: bool,
    pub flag_uppercase_umlauts: bool,
    pub flag_digits: bool,
    pub flag_common_special_chars: bool,
    pub flag_all_special_chars: bool,
}

impl CliArgs {
    fn new() -> CliArgs {
        CliArgs {
            input_to_crack: None,
            hashing_algo: None,
            max_len: None,
            min_len: None,
            flag_show_help: false,
            flag_lowercase_letters: false,
            flag_uppercase_letters: false,
            flag_lowercase_umlauts: false,
            flag_uppercase_umlauts: false,
            flag_digits: false,
            flag_common_special_chars: false,
            flag_all_special_chars: false
        }
    }
}