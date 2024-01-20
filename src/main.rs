#![warn(unused_variables)]
#![allow(dead_code)]
mod cli;
mod cli_ops;
mod display;
mod factorisations;
mod groups_modulo_n;
mod logarithms;
mod presets;
mod primality;
mod prime_factors;
mod quadratic_sieve;
mod utils;

use std::{clone, collections::HashMap, io::Write};

use factorisations::pollards_p_1;
use json_to_table::json_to_table;
use num_iter::range_inclusive;
use quadratic_sieve::prepare_matrix;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tabled::{
    settings::{panel::Header, split::Split, Style},
    Table,
};

use cli::{cli, CarmichaelMethods, PrimalityMethods};
use fmtastic::Superscript;
use serde_json::json;

use display::{matrix_print, Matrix};
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use presets::{
    find_primes_in_range_trial_division_parallel, list_carmichael_nums, list_prime_factors_in_range,
};
use primality::{aks, carmichael_nums_flt, carmichael_nums_korselt, gcd_test};
use utils::findr;

use crate::{
    display::{NumFactorTable, P_k_2P_kTable, PrimitiveRootsTable},
    groups_modulo_n::{
        euler_totient_phi, is_integer_of_form_pk_2pk, primitive_roots_trial_n_error,
    },
    presets::{find_miller_rabin_liars, search_nums_with_primitive_roots, NumCategory},
    primality::{is_prime_trial_division_parallel, miller_rabin_primality, AksSteps},
    prime_factors::PrimeFactors,
    utils::{modular_pow, Gcd},
};

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            let help = cli().render_long_help();
            println!("{help}");
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("primes", matches)) => {
            let s = matches.get_one::<BigInt>("start").expect("required");
            let e = matches.get_one::<BigInt>("end").expect("required");
            let (primes, _) = find_primes_in_range_trial_division_parallel(s.clone(), e.clone());

            let table_data = &primes
                .iter()
                .map(|x| Matrix::new(x.to_string()))
                .collect::<Vec<Matrix>>();
            matrix_print(table_data, "Prime Numbers:".to_string(), &primes.len() / 5);
            //std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("composites", matches)) => {
            let s = matches.get_one::<BigInt>("start").expect("required");
            let e = matches.get_one::<BigInt>("end").expect("required");
            let (_, composites) =
                find_primes_in_range_trial_division_parallel(s.clone(), e.clone());

            let table_data = &composites
                .iter()
                .map(|x| Matrix::new(x.to_string()))
                .collect::<Vec<Matrix>>();
            matrix_print(
                table_data,
                "Prime Numbers:".to_string(),
                &composites.len() / 14,
            );
            //std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("composites-pq", matches)) => {
            let s = matches.get_one::<BigInt>("start").expect("required");
            let e = matches.get_one::<BigInt>("end").expect("required");

            let mut composites = list_prime_factors_in_range(s, &e, NumCategory::CompositesPQ).1;
            // filter only odd composite numbers with only two factors
            composites.retain(|(num, _)| num % 2 != BigInt::zero());

            let table_data = &composites
                .iter()
                .map(|x| Matrix::new(x.0.to_string()))
                .collect::<Vec<Matrix>>();
            matrix_print(
                table_data,
                "Composites N = P.Q:".to_string(),
                &composites.len() / 14,
            );
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("nums-with-primitive-roots", matches)) => {
            let s = matches.get_one::<BigInt>("start").expect("required");
            let e = matches.get_one::<BigInt>("end").expect("required");

            let (nums_with_prim_roots, nums_without_no_prim_roots) =
                search_nums_with_primitive_roots(s.clone(), e.clone());

            println!(
                "{}",
                serde_json::to_string(&HashMap::from([(
                    "Numbers With Primitve Roots".to_string(),
                    &nums_with_prim_roots
                )]))
                .unwrap()
            );

            println!("");
            println!(
                "{}",
                serde_json::to_string(&HashMap::from([(
                    "Numbers Without Primitive Roots".to_string(),
                    &nums_without_no_prim_roots
                )]))
                .unwrap()
            );

            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("carmichael-nums", matches)) => {
            let s = matches.get_one::<BigInt>("start").expect("required");
            let e = matches.get_one::<BigInt>("end").expect("required");
            let method = matches
                .get_one::<CarmichaelMethods>("method")
                .expect("required");
            match method {
                &CarmichaelMethods::Fermat => {
                    let carmichael_nums = list_carmichael_nums(s, e, carmichael_nums_flt);
                    println!("\n{}\n", carmichael_nums.0);
                }
                CarmichaelMethods::Korselt => {
                    let carmichael_nums = list_carmichael_nums(s, e, carmichael_nums_korselt);
                    println!("\n{}\n", carmichael_nums.0);
                }
            }
            // let (primes, _) = find_primes_in_range_trial_division_parallel(s.clone(), e.clone());

            // let table_data = &primes
            //     .iter()
            //     .map(|x| Matrix::new(x.to_string()))
            //     .collect::<Vec<Matrix>>();
            // matrix_print(table_data, "Prime Numbers:".to_string(), &primes.len() / 5);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("ifactors", matches)) => {
            let n1 = matches.get_one::<BigInt>("NUM1").expect("required");
            if let Some(n2) = matches.get_one::<BigInt>("NUM2") {
                if let Some(pq) = matches.get_one::<bool>("PQ") {
                    if *pq {
                        let num_pfactors =
                            list_prime_factors_in_range(n1, n2, NumCategory::CompositesPQ);
                        matrix_print(
                            &num_pfactors.0,
                            "Prime Factorisation - Composites of the form N = P.Q:".to_string(),
                            &num_pfactors.0.len() / 4,
                        );
                        println!("PQ: {}", pq);
                    } else {
                        let num_pfactors =
                            list_prime_factors_in_range(n1, n2, NumCategory::Composites);
                        matrix_print(
                            &num_pfactors.0,
                            "Prime Factorisation - Only Composites In The Range:".to_string(),
                            &num_pfactors.0.len() / 4,
                        );
                    }
                }
            } else {
                let mut primes = vec![BigInt::from(2u64)];
                println!("{:?}", n1.prime_factors(&mut primes));
            }

            //std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("primality", matches)) => {
            let method = matches
                .get_one::<PrimalityMethods>("method")
                .expect("required");
            let n = matches.get_one::<BigInt>("num").expect("required");
            match method {
                PrimalityMethods::TrialDivision => {
                    if is_prime_trial_division_parallel(n) {
                        println!("{} is Prime", n);
                    } else {
                        println!("{} is Composite", n);
                    }
                }
                PrimalityMethods::Fermat => {
                    println!("Fermat Primality Test - Not Implemented!");
                }
                PrimalityMethods::Gcd => {
                    let res = gcd_test(n, 5);
                    let mut composite = false;
                    for i in res.iter() {
                        if i.1 > BigInt::one() {
                            composite = true;
                            println!("GCD Test: {} is Composite.", n);
                        }
                    }
                    if !composite {
                        println!("GCD Test: {} is Prime.", n);
                    }
                }
                PrimalityMethods::MillerRabin => {
                    if miller_rabin_primality(n) {
                        println!("{} is Probably Prime", n);
                    } else {
                        println!("{} is Definitely Composite", n);
                    }
                }
                PrimalityMethods::AKS => {
                    if aks(n).0 {
                        println!("{} is Prime", n);
                    } else {
                        println!("{} is Composite", n);
                    }
                }
            }
        }
        Some(("miller-rabin-liars", matches)) => {
            let n = matches.get_one::<BigInt>("num").expect("required");
            let mut primes = vec![BigInt::from(2u64)];
            let p_factors = n.prime_factors(&mut primes);
            // call miller-rabin test
            let (_, non_witnesses) = find_miller_rabin_liars(n);
            // Convert prime factors to String format
            let mut form = String::new();
            for (factor, exp) in p_factors {
                form.push_str(&format!("{}{} x ", factor, Superscript(exp.clone())));
            }
            let mut form = form.trim_end().to_string();
            form.pop();
            let json = json!(&non_witnesses);
            let mut table = json_to_table(&json).into_table();

            table.with(Style::modern()).with(Split::row(5).concat());
            println!("\nMiller-Rabin Liars for {} = {}", n, form);
            println!("{table}\n");
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("quadratic-sieve", matches)) => {
            let n = matches.get_one::<BigInt>("NUM").expect("required");
            prepare_matrix(&n);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("pollards-p-minus-1", matches)) => {
            let n = matches.get_one::<BigInt>("NUM").expect("required");
            let b = matches.get_one::<BigInt>("BASE").expect("required");
            pollards_p_1(n, b);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("pollards-rho", matches)) => {
            let r = matches
                .get_one::<BigInt>("primitive-root")
                .expect("required");
            let b = matches.get_one::<BigInt>("b").expect("required");
            let m = matches.get_one::<BigInt>("modulo").expect("required");
            logarithms::pollards_rho(&r, &b, &m);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("gcd", matches)) => {
            let a = matches.get_one::<BigInt>("NUM1").expect("required");
            let b = matches.get_one::<BigInt>("NUM2").expect("required");
            println!("\ngcd({}, {}) = {}\n", a, b, a.gcd_euclid(&b));
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("aks-findr", matches)) => {
            let n = matches.get_one::<BigInt>("NUM").expect("required");
            let r = findr(n);
            println!("\nAKS 'r' value for {} is = {}", n, r);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("list-primitive-roots", matches)) => {
            let n = matches.get_one::<BigInt>("NUM").expect("required");
            let primitive_roots = primitive_roots_trial_n_error(&n);
            println!(
                "\nPrimitive Roots of n = {}: \n\t{:?}\n",
                n, primitive_roots
            );
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("modular-pow", matches)) => {
            let b = matches.get_one::<BigInt>("base").expect("required");
            let e = matches.get_one::<BigInt>("exponent").expect("required");
            let m = matches.get_one::<BigInt>("modulo").expect("required");
            let result = modular_pow(b, e, &m);
            let result = format!(
                "{}{}(mod {}) = {}",
                b.to_string(),
                Superscript(e.to_u32().unwrap()),
                m,
                result
            );
            println!("\n{}\n", result);
            // println!("{}", serde_json::to_string_pretty(&result).unwrap());
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("ass2q2b", matches)) => {
            let start = matches.get_one::<BigInt>("start").expect("required");
            let end = matches.get_one::<BigInt>("end").expect("required");

            let mut result: Vec<PrimitiveRootsTable> = Vec::new();
            let (primes_in_range, _) =
                find_primes_in_range_trial_division_parallel(start.clone(), end.clone());
            for p in primes_in_range.iter() {
                let primitive_roots = primitive_roots_trial_n_error(p);
                let phi_phi_n = euler_totient_phi(&(p - BigInt::one()));
                let row = PrimitiveRootsTable::new(
                    p.to_string(),
                    phi_phi_n.to_string(),
                    primitive_roots.len().to_string(),
                );
                result.push(row);
            }
            let mut table = Table::new(&result);

            table.with(Style::modern());
            println!("\n{}\n", table.to_string());
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("ass2q2c", matches)) => {
            let start = matches.get_one::<BigInt>("start").expect("required");
            let end = matches.get_one::<BigInt>("end").expect("required");

            let mut result: Vec<P_k_2P_kTable> = Vec::new();
            for n in range_inclusive(start.clone(), end.clone()) {
                let p_factors = is_integer_of_form_pk_2pk(&n);
                if !p_factors.is_empty() {
                    let mut form: String = String::new();
                    for (factor, exp) in p_factors {
                        form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
                    }
                    let mut form = form.trim_end().to_string();
                    form.pop();
                    result.push(P_k_2P_kTable::new(n.to_string(), form));
                }
            }

            let mut table = Table::new(&result);
            table.with(Style::modern());
            println!("\nNumbers of the form pᵏ, 2pᵏ:");
            println!("{table}\n");
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("ass2q3d", matches)) => {
            let start = matches.get_one::<BigInt>("start").expect("required");
            let end = matches.get_one::<BigInt>("end").expect("required");

            let (_, nums_without_no_prim_roots) =
                search_nums_with_primitive_roots(start.clone(), end.clone());

            let num_pfactors = list_prime_factors_in_range(&start, &end, NumCategory::CompositesPQ);

            let mut num_map: Vec<NumFactorTable> = Vec::new();
            for (num, factors) in num_pfactors.1 {
                if let Ok(_) = nums_without_no_prim_roots.binary_search(&num.to_string()) {
                    let mut form: String = String::new();
                    for (factor, exp) in factors {
                        form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
                    }
                    let mut form = form.trim_end().to_string();
                    form.pop();
                    // let v: Vec<(String, String)> = factor
                    //     .iter()
                    //     .map(|(i, j)| (i.to_string(), j.to_string()))
                    //     .collect::<Vec<(String, String)>>();
                    num_map.push(NumFactorTable::new(num.to_string(), form));
                }
            }

            let json = json!(&nums_without_no_prim_roots);
            let mut table1 = json_to_table(&json).into_table();

            table1.with(Style::modern()).with(Split::row(5).concat());
            println!("\nSet of numbers without primitive roots:");
            println!("{table1}\n");

            let mut table2 = Table::new(&num_map);
            table2.with(Style::modern()).with(Split::row(5).concat());
            println!("\nNumbers of the form N = P.Q is included in the above table:");
            println!("{table2}\n");
        }
        Some(("aks-failed-steps-for-n", matches)) => {
            let start = matches.get_one::<BigInt>("start").expect("required");
            let end = matches.get_one::<BigInt>("end").expect("required");

            let mut result: HashMap<String, Vec<String>> = HashMap::new();
            let composites = list_prime_factors_in_range(start, end, NumCategory::Composites).1;
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

            for (k, v) in result.iter() {
                let json = json!(v);
                let mut table1 = json_to_table(&json).into_table();

                table1.with(Style::modern()).with(Split::row(5).concat());
                println!("\nNumbers below failed in {} of AKS Algm:", k);
                println!("{table1}\n");
            }

            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("quit", _matches)) => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "nt-tools> ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
