mod primality;
mod prime_factors;
mod presets;
mod utils;
mod display;

use num_bigint::BigInt;
use clap::{Args, Parser, Subcommand, ValueEnum};
use presets::{find_primes_in_range_trial_division_parallel, list_prime_factors_in_range, list_prime_factors_in_range_form_pq, gcd_test_range};
use fmtastic::{Subscript, Superscript};
use primality::gcd_test;
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
    /// Search for the prime numbers in a range
    #[command(arg_required_else_help = true)]
    ListPrimes {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: BigInt,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: BigInt,
    },
    /// Find the prime factorisation of a number
    #[command(arg_required_else_help = true)]
    PrimeFactors {
        #[arg(short = 'n', long = "num", value_name = "NUMBER")]
        num: BigInt,
    },
    /// Find the prime factorisation of a range of numbers
    #[command(arg_required_else_help = true)]
    PrimesFactorsRange {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: BigInt,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: BigInt,
    },
    /// Search composite numbers of the form p.q where p and q are primes
    #[command(arg_required_else_help = true)]
    PrimesFactorsPQ {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: BigInt,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: BigInt,
    },
    /// Primality checking
    #[command(arg_required_else_help = true)]
    Primality(PrimalityArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct PrimalityArgs {
    #[command(subcommand)]
    command: PrimalityCommands,
}

#[derive(Debug, Subcommand)]
enum PrimalityCommands {
    GCD(PrimalityRangeArgs),
    MillerRabin(PrimalityRangeArgs),
}

#[derive(Debug, Args)]
struct PrimalityRangeArgs {
    #[arg(short, long, value_name = "This must be present if primality check is for a single number")]
    num: Option<BigInt>,
    
    #[arg(short = 's', long = "start", value_name = "START NUMBER for a range of numbers")]
    start: Option<BigInt>,

    #[arg(short = 'e', long = "end", value_name = "END NUMBER for the range")]
    end: Option<BigInt>,
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
            let pq_nums = list_prime_factors_in_range_form_pq(&start, &end);
            println!("{}", pq_nums.0);
        },
        Operations::Primality(s) => {
            match s.command  {
                PrimalityCommands::GCD(gcd_test_args) => {
                    if let Some(num) = gcd_test_args.num {
                        gcd_test(&num, 5);
                    } else {
                        if let Some(start) = gcd_test_args.start {
                            if let Some(end) = gcd_test_args.end {
                                gcd_test_range(&start, &end);
                            }
                        }
                    }
                },
                PrimalityCommands::MillerRabin(miller_rabin_args) => println!("{:?}", miller_rabin_args),
            }
        }
    }
}