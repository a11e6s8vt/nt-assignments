mod cli_ops;
mod display;
mod presets;
mod primality;
mod prime_factors;
mod utils;

use clap::{Args, Parser, Subcommand, ValueEnum};
use cli_ops::{Cli, Operations, PFactorsCommands, PrimalityCommands};
use fmtastic::{Subscript, Superscript};
use num_bigint::BigInt;
use presets::{
    find_primes_in_range_trial_division_parallel, gcd_test_range, list_prime_factors_in_range,
};
use primality::gcd_test;
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

use crate::presets::NumCategory;

fn main() {
    let args = Cli::parse();

    match args.command {
        Operations::ListPrimes { start, end } => {
            find_primes_in_range_trial_division_parallel(start, end);
        }
        Operations::PrimeFactors { num } => {}
        Operations::PrimesFactorsRange(s) => match s.command {
            PFactorsCommands::All(pargs) => {
                let start = pargs.start;
                let end = pargs.end;

                let num_pfactors = list_prime_factors_in_range(&start, &end, NumCategory::All);
                println!("\n{}\n", num_pfactors.0);
            }
            PFactorsCommands::Composites(pargs) => {
                let start = pargs.start;
                let end = pargs.end;

                let num_pfactors =
                    list_prime_factors_in_range(&start, &end, NumCategory::Composites);
                println!("\n{}\n", num_pfactors.0);
            }
            PFactorsCommands::CompositesPQ(pargs) => {
                let start = pargs.start;
                let end = pargs.end;

                let num_pfactors =
                    list_prime_factors_in_range(&start, &end, NumCategory::CompositesPQ);
                println!("\n{}\n", num_pfactors.0);
            }
        },
        Operations::Primality(s) => match s.command {
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
            }
            PrimalityCommands::MillerRabin(miller_rabin_args) => {
                println!("{:?}", miller_rabin_args)
            }
        },
    }
}
