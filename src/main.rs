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

use std::io::Write;

use clap::{arg, Arg, ArgAction, ArgGroup, Command};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    primitive,
};

use clap::{builder::Str, Parser};
use cli_ops::{
    CarmichaelNumsCommands, Cli, Operations, PFactorsCommands, PrimalityCommands,
    PrimitiveRootsCommands,
};
use factorisations::pollards_p_1;
use fmtastic::Superscript;
use groups_modulo_n::{
    euler_totient_phi, is_integer_of_form_pk_2pk, primitive_roots_trial_n_error,
};
use homedir::get_my_home;
use num_iter::range_inclusive;
use quadratic_sieve::prepare_matrix;

use display::{matrix_print, Matrix};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use presets::{
    find_primes_in_range_trial_division_parallel, gcd_test_range, list_carmichael_nums,
    list_prime_factors_in_range,
};
use primality::{aks, carmichael_nums_flt, carmichael_nums_korselt, gcd_test};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::findr;

use crate::{
    display::MillerRabinJson,
    presets::{ass1_question3_miller_rabin, search_nums_with_primitive_roots, NumCategory},
    primality::{
        is_prime_trial_division_parallel, miller_rabin_primality, AksSteps, PrimalityMethods,
    },
    prime_factors::PrimeFactors,
    utils::{modular_pow, Gcd},
};

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
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
            let n1 = matches.get_one::<BigInt>("num1").expect("required");
            match method {
                PrimalityMethods::TrialDivision => {}
                PrimalityMethods::Fermat => {}
                PrimalityMethods::Gcd => {
                    if let Some(n2) = matches.get_one::<BigInt>("num2") {
                        gcd_test_range(n1, n2);
                    } else {
                        let res = gcd_test(n1, 5);
                        let mut composite = false;
                        for i in res.iter() {
                            if i.1 > BigInt::one() {
                                composite = true;
                                println!("GCD Test: {} is Composite.", n1);
                            }
                        }
                        if !composite {
                            println!("GCD Test: {} is Prime.", n1);
                        }
                    }
                }
                PrimalityMethods::MillerRabin => {
                    if let Some(n2) = matches.get_one::<BigInt>("num2") {
                        todo!()
                    } else {
                        if miller_rabin_primality(n1) {
                            println!("{} is Prime", n1);
                        } else {
                            println!("{} is Composite", n1);
                        }
                    }
                }
                PrimalityMethods::AKS => {}
            }
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

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("primes")
                .arg(
                    arg!(-s --start <START>)
                        .required(true)
                        .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(
                    arg!(-e --end <END>)
                        .required(true)
                        .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Search for prime numbers between START and END numbers")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("composites")
                .arg(
                    arg!(-s --start <START>)
                        .required(true)
                        .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(
                    arg!(-e --end <END>)
                        .required(true)
                        .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Search for composite numbers between START and END numbers")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("composites-pq")
                .arg(
                    arg!(-s --start <START>)
                        .required(true)
                        .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(
                    arg!(-e --end <END>)
                        .required(true)
                        .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Search for composite numbers of the form \"p.q\" between START and END numbers")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("ifactors")
                .arg(Arg::new("NUM1")
                    .short('a')
                    .long("num1")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(Arg::new("NUM2")
                    .short('b')
                    .long("num2")
                    .required(false)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(Arg::new("PQ")
                    .long("pq")
                    .action(ArgAction::SetTrue)
                    .required(false)
                    .num_args(0)
                    .requires("NUM2")
                )
                .about("Find the Integer Factorisation of a number. If the second argument is passed, 
                            it prints the factorisation of all numbers in the range.")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("primality")
                .arg(Arg::new("method")
                    .long("method")
                    .required(true)
                    .value_parser(clap::builder::EnumValueParser::<PrimalityMethods>::new())
                )
                .arg(Arg::new("num1")
                    .short('a')
                    .long("num1")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt))
                )
                .arg(Arg::new("num2")
                    .short('b')
                    .long("num2")
                    .required(false)
                    .value_parser(clap::value_parser!(BigInt))
                )
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
