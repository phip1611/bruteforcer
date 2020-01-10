use libbruteforce::transformation_fns::TransformationFn;
use crate::args::CliArgs;
use libbruteforce::transformation_fns::identity::NO_HASHING;

pub struct ProgramOptions {
    algo: TransformationFn
}

impl ProgramOptions {
    fn from(args: &CliArgs) -> ProgramOptions {
        ProgramOptions {
            algo: NO_HASHING
        }
    }
}