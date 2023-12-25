mod primality;
mod presets;

use num_bigint::BigInt;
use clap::{ArgAction, Parser, Subcommand};
use presets::find_primes_in_range_trial_division_parallel;
use std::ops::Range;
use fmtastic::{Subscript, Superscript};
use num_iter::range_inclusive;


#[derive(Debug, Parser)]
#[command(
    author = "Ajeesh T. Vijayan", 
    version, 
    about = "Number Theory Calculator", 
    long_about = None
)]

struct Cli {
    #[command(subcommand)]
    command: Operations,
}

#[derive(Debug, Subcommand)]
enum Operations {
    #[command(arg_required_else_help = true)]
    ListPrimes {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: BigInt,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: BigInt,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Operations::ListPrimes { start, end } => {
            find_primes_in_range_trial_division_parallel(start, end);
        }
    }
}