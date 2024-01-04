mod cli_ops;
mod display;
mod groups_modulo_n;
mod presets;
mod primality;
mod prime_factors;
mod utils;

use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
};

use clap::Parser;
use cli_ops::{CarmichaelNumsCommands, Cli, Operations, PFactorsCommands, PrimalityCommands};
use fmtastic::Superscript;
use homedir::get_my_home;
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
use utils::findr;

use crate::{
    display::MillerRabinJson,
    presets::NumCategory,
    primality::{is_prime_trial_division_parallel, AksSteps},
    utils::modular_pow,
};

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
            // filter only odd composite numbers with only two factors
            // composites.retain(|(num, p_factors)| p_factors.len() == 2 && num % 2 != BigInt::zero());
            composites.retain(|(num, p_factors)| num % 2 != BigInt::zero());
            // take the first five elements for the test
            // let sample_data = &composites[0..5];
            println!(
                "Total Number of Odd Composites with two factors {}",
                &composites.len()
            );
            let mut json_out: BTreeMap<String, MillerRabinJson> = BTreeMap::new();
            for (num, p_factors) in composites.iter() {
                println!("Processing the number: {}", num);
                // call miller-rabin test
                let (n_minus_one_form, non_witnesses) = test_primality_miller_rabin(num);
                // Convert prime factors to String format
                let mut form = String::new();
                for (factor, exp) in p_factors {
                    form.push_str(&format!("{}{} x ", factor, Superscript(exp.clone())));
                }
                let mut form = form.trim_end().to_string();
                form.pop();
                if !non_witnesses.is_empty() {
                    let mr_json = MillerRabinJson::new(n_minus_one_form, form, non_witnesses);
                    json_out.insert(num.to_string(), mr_json);
                }
            }

            let my_home = get_my_home()
                .unwrap()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let mut output_dir = String::new();
            let mut fname = String::new();

            if cfg!(windows) {
                output_dir.push_str(&my_home);
                output_dir.push_str("\\ass1-question3");
                println!("Path = {}", &output_dir);
                fname.push_str(&output_dir);
                fname.push_str("\\");
                fname.push_str("question3.json");
            } else if cfg!(unix) {
                output_dir.push_str(&my_home);
                output_dir.push_str("/ass1-question3");
                println!("Path = {}", &output_dir);
                fname.push_str(&output_dir);
                fname.push_str("/");
                fname.push_str("question3.json");
            }
            println!("output dir: {}", &output_dir);
            if !fs::metadata(&output_dir).is_ok() {
                let _ = fs::create_dir(&output_dir);
            }
            match File::create(&fname) {
                Ok(file) => {
                    println!("Output has been written to the file: {}", &fname);
                    serde_json::to_writer_pretty(file, &json_out).unwrap();
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
        }
        Operations::AKS(s) => {
            let mut result: HashMap<String, Vec<String>> = HashMap::new();
            let composites =
                list_prime_factors_in_range(&s.start, &s.end, NumCategory::Composites).1;
            let aks_test_res = composites
                .par_iter()
                .map(|(num, _)| (num, aks(num)))
                .map(|(num, (is_prime, step))| (num.clone(), is_prime, step))
                .collect::<Vec<(BigInt, bool, AksSteps)>>();
            for (num, is_prime, step) in aks_test_res.iter() {
                match step {
                    AksSteps::Step1 => {
                        if !is_prime {
                            if result.get("step1").is_some() {
                                let s = result.get_mut("step1").unwrap();
                                s.push(num.to_string());
                            } else {
                                result.insert("step1".to_string(), vec![num.to_string()]);
                            }
                        }
                    }
                    AksSteps::Step2 => {
                        if !is_prime {
                            if result.get("step2").is_some() {
                                let s = result.get_mut("step2").unwrap();
                                s.push(num.to_string());
                            } else {
                                result.insert("step2".to_string(), vec![num.to_string()]);
                            }
                        }
                    }
                    AksSteps::Step3 => {
                        if !is_prime {
                            if result.get("step3").is_some() {
                                let s = result.get_mut("step3").unwrap();
                                s.push(num.to_string());
                            } else {
                                result.insert("step3".to_string(), vec![num.to_string()]);
                            }
                        }
                    }
                    AksSteps::Step4 => {
                        if !is_prime {
                            if result.get("step4").is_some() {
                                let s = result.get_mut("step4").unwrap();
                                s.push(num.to_string());
                            } else {
                                result.insert("step4".to_string(), vec![num.to_string()]);
                            }
                        }
                    }
                    AksSteps::Step5 => {
                        if !is_prime {
                            if result.get("step5").is_some() {
                                let s = result.get_mut("step5").unwrap();
                                s.push(num.to_string());
                            } else {
                                result.insert("step5".to_string(), vec![num.to_string()]);
                            }
                        }
                    }
                    AksSteps::Success => {
                        if *is_prime {
                            if result.get("success").is_some() {
                                let s = result.get_mut("success").unwrap();
                                s.push(num.to_string());
                            } else {
                                result.insert("success".to_string(), vec![num.to_string()]);
                            }
                        }
                    }
                }
            }

            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Operations::ModularPower {
            base,
            exponent,
            modulus,
        } => {
            println!("{}", modular_pow(&base, &exponent, &modulus));
        }
        Operations::FindrAKS { num } => {
            let r = findr(&num);
            println!("AKS 'r' value for {} is = {}", num, r);
        }
    }
}
