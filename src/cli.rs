use clap::{arg, Arg, ArgAction, Command, ValueHint};
use num_bigint::BigInt;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PrimalityMethods {
    TrialDivision,
    Fermat,
    Gcd,
    MillerRabin,
    AKS,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CarmichaelMethods {
    Fermat,
    Korselt,
}

pub fn cli() -> Command {
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
        .subcommand_value_name("NumberTheoryTools")
        .subcommand_help_heading("NumberTheoryTools")
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
            Command::new("nums-with-primitive-roots")
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
                .about("Search for numbers with primitive roots between START and END numbers")
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
            Command::new("gcd")
                .arg(Arg::new("NUM1")
                    .short('a')
                    .long("num1")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(Arg::new("NUM2")
                    .short('b')
                    .long("num2")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Finds the GCD of two numbers using Euclid's algorithm.")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("quadratic-sieve")
                .arg(Arg::new("NUM")
                    .short('n')
                    .long("num")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Integer Factorisation - Quadratic Sieve.")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("pollards-p-minus-1")
                .arg(Arg::new("NUM")
                    .short('n')
                    .long("num")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(Arg::new("BASE")
                    .short('b')
                    .long("base")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Integer Factorisation - Pollard's P-1 Algm.")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("pollards-rho")
                .arg(Arg::new("primitive-root")
                    .short('r')
                    .long("primitive-root")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt))
                    .help("Primitive Root modulo N"),
                )
                .arg(Arg::new("b")
                    .short('b')
                    .required(true)
                    .help("b ∈ Z/pZ - Find the logarithm of b to the base r")
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(Arg::new("modulo")
                    .short('m')
                    .long("modulo")
                    .required(true)
                    .help("Odd Prime Number")
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Pollards Rho Alogorithm to find the logarithm modulo p")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("modular-pow")
                .arg(Arg::new("base")
                    .short('b')
                    .long("base")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt))
                    .help("Base number which we are raising to some power"),
                )
                .arg(Arg::new("exponent")
                    .short('e')
                    .required(true)
                    .help("Exponent")
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .arg(Arg::new("modulo")
                    .short('m')
                    .long("modulo")
                    .required(true)
                    .help("Modulus")
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Fast Modular Exponentiation")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("aks-findr")
                .arg(Arg::new("NUM")
                    .short('n')
                    .long("num")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("Finds the 'r' value for the AKS algorithm.")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("list-primitive-roots")
                .arg(Arg::new("NUM")
                    .short('n')
                    .long("num")
                    .required(true)
                    .value_parser(clap::value_parser!(BigInt)),
                )
                .about("List the primitive roots of a number")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("ass2q2b")
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
                .about("Assignment 2 - Question 2b - Primitive Roots - Euler's Totient Function")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("ass2q2c")
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
                .about("Assignment 2 - Question 2c - Primitive Roots - Euler's Totient Function")
                .help_template(APP_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .help_template(APP_TEMPLATE),
        );
    cmd
}
