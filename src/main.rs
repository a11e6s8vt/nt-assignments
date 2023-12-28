mod cli_ops;
mod display;
mod display;
mod presets;
mod primality;
mod prime_factors;
mod utils;

use clap::{Args, Parser, Subcommand, ValueEnum};
use cli_ops::{CarmichaelNumsCommands, Cli, Operations, PFactorsCommands, PrimalityCommands};
use display::miller_rabin_output_print;
use fmtastic::{Subscript, Superscript};
use num_bigint::BigInt;
use num_traits::Zero;
use presets::{
    find_primes_in_range_trial_division_parallel, gcd_test_range, list_carmichael_nums,
    list_prime_factors_in_range, test_primality_miller_rabin,
};
use primality::{carmichael_nums_flt, carmichael_nums_korselt, gcd_test};
use rand::distributions::uniform::UniformSampler;
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
        Operations::CarmichaelNums(s) => match s.command {
            CarmichaelNumsCommands::Korselt(cargs) => {
                let start = cargs.start;
                let end = cargs.end;
                let carmichael_nums = list_carmichael_nums(&start, &end, carmichael_nums_korselt);
                println!("\n{}\n", carmichael_nums.0);
            }
            CarmichaelNumsCommands::FermatLT(cargs) => {
                let start = cargs.start;
                let end = cargs.end;
                let carmichael_nums = list_carmichael_nums(&start, &end, carmichael_nums_flt);
                println!("\n{}\n", carmichael_nums.0);
            }
        },
        Operations::Question3(s) => {
            let mut composites =
                list_prime_factors_in_range(&s.start, &s.end, NumCategory::Composites).1;
            // composite numbers with only two factors
            composites.retain(|(num, p_factors)| p_factors.len() == 2 && num % 2 != BigInt::zero());
            let sample_data = &composites[0..5];

            for (num, p_factors) in sample_data.iter() {
                test_primality_miller_rabin(num, 5);
            }
        }
    }
}
