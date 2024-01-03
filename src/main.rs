mod cli_ops;
mod display;
mod groups_modulo_n;
mod presets;
mod primality;
mod prime_factors;
mod utils;

use clap::Parser;
use cli_ops::{CarmichaelNumsCommands, Cli, Operations, PFactorsCommands, PrimalityCommands};
use serde_json::Result;

use display::{matrix_print, Matrix};
use num_bigint::BigInt;
use num_traits::Zero;
use presets::{
    find_primes_in_range_trial_division_parallel, gcd_test_range, list_carmichael_nums,
    list_prime_factors_in_range, test_primality_miller_rabin,
};
use primality::{aks, carmichael_nums_flt, carmichael_nums_korselt, gcd_test};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{presets::NumCategory, utils::modular_pow};

fn main() {
    let args = Cli::parse();

    match args.command {
        Operations::ListPrimes(s) => {
            let (primes, _) = find_primes_in_range_trial_division_parallel(s.start, s.end);

            let table_data = &primes
                .iter()
                .map(|x| Matrix::new(x.to_string()))
                .collect::<Vec<Matrix>>();
            matrix_print(table_data, "Prime Numbers:".to_string(), &primes.len() / 5);
        }
        Operations::ListComposites(s) => {
            let (_, composites) = find_primes_in_range_trial_division_parallel(s.start, s.end);

            let table_data = &composites
                .iter()
                .map(|x| Matrix::new(x.to_string()))
                .collect::<Vec<Matrix>>();
            matrix_print(
                table_data,
                "Prime Numbers:".to_string(),
                &composites.len() / 14,
            );
        }
        Operations::PrimeFactors { num: _ } => {}
        Operations::PrimeFactorsRange(s) => match s.command {
            PFactorsCommands::All(pargs) => {
                let start = pargs.start;
                let end = pargs.end;

                let num_pfactors = list_prime_factors_in_range(&start, &end, NumCategory::All);

                matrix_print(
                    &num_pfactors.0,
                    "Prime Factorisation - All Numbers In The Range:".to_string(),
                    &num_pfactors.0.len() / 4,
                );
            }
            PFactorsCommands::Composites(pargs) => {
                let start = pargs.start;
                let end = pargs.end;

                let num_pfactors =
                    list_prime_factors_in_range(&start, &end, NumCategory::Composites);
                matrix_print(
                    &num_pfactors.0,
                    "Prime Factorisation - Only Composites In The Range:".to_string(),
                    &num_pfactors.0.len() / 4,
                );
                //let json = serde_json::to_string(&num_pfactors.0).unwrap();
                //println!("{}", json);
                // println!("\n{}\n", num_pfactors.0);
            }
            PFactorsCommands::CompositesPQ(pargs) => {
                let start = pargs.start;
                let end = pargs.end;

                let num_pfactors =
                    list_prime_factors_in_range(&start, &end, NumCategory::CompositesPQ);
                matrix_print(
                    &num_pfactors.0,
                    "Prime Factorisation - Composites of the form N = P.Q:".to_string(),
                    &num_pfactors.0.len() / 4,
                );
                //let json = serde_json::to_string(&num_pfactors.0).unwrap();
                //println!("{}", json);
                // println!("\n{}\n", num_pfactors.0);
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
            //println!("{:?}", composites.len());
            // let sample_data = &composites[1..2];

            // for (num, _p_factors) in sample_data.iter() {
            //     test_primality_miller_rabin(num, 1);
            // }
            println!("{:?}", &composites);
            println!("{:?}", &composites.len());
            for (num, _p_factors) in composites.iter() {
                test_primality_miller_rabin(num, 1);
            }
        }
        Operations::AKS(s) => {
            let composites =
                list_prime_factors_in_range(&s.start, &s.end, NumCategory::Composites).1;
            let aks_test_res = composites
                .par_iter()
                .map(|(num, _)| (num, aks(num)))
                .map(|(num, is_prime)| (num.clone(), is_prime))
                .collect::<Vec<(BigInt, bool)>>();
            println!("{:?}", aks_test_res);
        }
        Operations::ModularPower {
            base,
            exponent,
            modulus,
        } => {
            println!("{}", modular_pow(&base, &exponent, &modulus));
        }
    }
}
