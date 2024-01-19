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

use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, Command, ValueHint};
use num_integer::Integer;
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
        is_prime_trial_division_parallel, miller_rabin_primality, AksSteps, CarmichaelMethods,
        PrimalityMethods,
    },
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
            // let (primes, _) = find_primes_in_range_trial_division_parallel(s.clone(), e.clone());

            // let table_data = &primes
            //     .iter()
            //     .map(|x| Matrix::new(x.to_string()))
            //     .collect::<Vec<Matrix>>();
            // matrix_print(table_data, "Prime Numbers:".to_string(), &primes.len() / 5);
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
                PrimalityMethods::AKS => {}
            }
        }
        Some(("miller-rabin-liars", matches)) => {
            let n = matches.get_one::<BigInt>("num").expect("required");
            let mut primes = vec![BigInt::from(2u64)];
            let p_factors = n.prime_factors(&mut primes);
            let mut json_out: BTreeMap<String, MillerRabinJson> = BTreeMap::new();
            // call miller-rabin test
            let (n_minus_one_form, non_witnesses) = ass1_question3_miller_rabin(n);
            // Convert prime factors to String format
            let mut form = String::new();
            for (factor, exp) in p_factors {
                form.push_str(&format!("{}{} x ", factor, Superscript(exp.clone())));
            }
            let mut form = form.trim_end().to_string();
            form.pop();
            let mr_json = MillerRabinJson::new(n_minus_one_form, form, non_witnesses);
            json_out.insert(n.to_string(), mr_json);

            println!("{}", serde_json::to_string_pretty(&json_out).unwrap());
            // let my_home = get_my_home()
            //     .unwrap()
            //     .unwrap()
            //     .to_str()
            //     .unwrap()
            //     .to_string();
            // let mut output_dir = String::new();
            // let mut fname = String::new();

            // if cfg!(windows) {
            //     output_dir.push_str(&my_home);
            //     output_dir.push_str("\\ass1-question3");
            //     println!("Path = {}", &output_dir);
            //     fname.push_str(&output_dir);
            //     fname.push_str("\\");
            //     fname.push_str("question3.json");
            // } else if cfg!(unix) {
            //     output_dir.push_str(&my_home);
            //     output_dir.push_str("/ass1-question3");
            //     println!("Path = {}", &output_dir);
            //     fname.push_str(&output_dir);
            //     fname.push_str("/");
            //     fname.push_str("question3.json");
            // }
            // println!("output dir: {}", &output_dir);
            // if !fs::metadata(&output_dir).is_ok() {
            //     let _ = fs::create_dir(&output_dir);
            // }
            // match File::create(&fname) {
            //     Ok(file) => {
            //         println!("Output has been written to the file: {}", &fname);
            //         serde_json::to_writer_pretty(file, &json_out).unwrap();
            //     }
            //     Err(e) => panic!("Problem creating the file: {:?}", e),
            // }
            // let (primes, _) = find_primes_in_range_trial_division_parallel(s.clone(), e.clone());

            // let table_data = &primes
            //     .iter()
            //     .map(|x| Matrix::new(x.to_string()))
            //     .collect::<Vec<Matrix>>();
            // matrix_print(table_data, "Prime Numbers:".to_string(), &primes.len() / 5);
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

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APP_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    let cmd = Command::new("nt-tools")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("NTAPP")
        .subcommand_help_heading("NTAPP")
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
                .help_template(APP_TEMPLATE),
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
                .help_template(APP_TEMPLATE),
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
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("carmichael-nums")
                .arg(Arg::new("method")
                    .long("method")
                    .required(true)
                    .value_parser(clap::builder::EnumValueParser::<CarmichaelMethods>::new())
                    .help("Choose the algorithm")
                )
                .arg(Arg::new("start")
                    .short('s')
                    .long("start")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt))
                )
                .arg(Arg::new("end")
                    .short('e')
                    .long("end")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Carmichael Number search in a range.")
                .help_template(APP_TEMPLATE),
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
                .about("Finds the Integer Factorisation of a number.")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("primality")
                .arg(Arg::new("method")
                    .long("method")
                    .required(true)
                    .value_parser(clap::builder::EnumValueParser::<PrimalityMethods>::new())
                    .help("Choose the primality Checking algorithm")
                )
                .arg(Arg::new("num")
                    .short('n')
                    .long("num")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt))
                )
                .about("Primality checking capabilities.")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("miller-rabin-liars")
                .arg(Arg::new("num")
                    .short('n')
                    .long("num")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt))
                )
                .about("List the Miller-Rabin Liars of a number if any exist")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .help_template(APP_TEMPLATE),
        );
    cmd
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
