mod primality;
mod prime_factors;
mod presets;
mod utils;

use num_bigint::BigInt;
use clap::{ArgAction, Parser, Subcommand};
use presets::{find_primes_in_range_trial_division_parallel, list_prime_factors_in_range, list_prime_factors_in_range_form_pq};
use fmtastic::{Subscript, Superscript};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};


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
    },
    #[command(arg_required_else_help = true)]
    PrimeFactors {
        #[arg(short = 'n', long = "num", value_name = "NUMBER")]
        num: BigInt,
    },
    #[command(arg_required_else_help = true)]
    PrimesFactorsRange {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: BigInt,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: BigInt,
    },
    #[command(arg_required_else_help = true)]
    PrimesFactorsPQ {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: BigInt,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: BigInt,
    },
}



fn main() {
    let args = Cli::parse();

    match args.command {
        Operations::ListPrimes { start, end } => {
            find_primes_in_range_trial_division_parallel(start, end);
        },
        Operations::PrimeFactors { num } => {

        },
        Operations::PrimesFactorsRange { start, end } => {
            list_prime_factors_in_range(&start, &end)
        },
        Operations::PrimesFactorsPQ { start, end } => {
            list_prime_factors_in_range_form_pq(&start, &end) 
        }
    }
}