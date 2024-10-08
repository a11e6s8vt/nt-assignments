use clap::{Args, Parser, Subcommand};
use num_bigint::BigInt;


#[derive(Debug, Parser)]
#[command(
    author = "Ajeesh T. Vijayan", 
    version, 
    about = "Number Theory Calculator", 
    long_about = None)] 
pub struct Cli {
    #[command(subcommand)]
    pub command: Operations,
}

#[derive(Debug, Subcommand)]
pub enum Operations {
    /// GCD of two number using Euclid's Algorithm
    #[command(arg_required_else_help = true)]
    GcdEuclid {
        #[arg(short = 'a', value_name = "FIRST NUMBER")]
        a: BigInt,

        #[arg(short = 'b', value_name = "SECOND NUMBER")]
        b: BigInt,
    },
    /// Pollard's p - 1 Method
    #[command(arg_required_else_help = true)]
    PollardsPOne {
        #[arg(short = 'n', value_name = "NUMBER to be factored")]
        n: BigInt,

        #[arg(short = 'a', value_name = "Initial base")]
        a: BigInt,
    },
    /// Quadratic Sieve
    #[command(arg_required_else_help = true)]
    QuadraticSieve {
        #[arg(short = 'n', value_name = "NUMBER")]
        n: BigInt,
    },
    /// Search for the prime numbers in a range
    #[command(arg_required_else_help = true)]
    ListPrimes(NumRangeArgs),
    
    /// Search for the composite numbers in a range
    #[command(arg_required_else_help = true)]
    ListComposites(NumRangeArgs),
    /// Find the prime factorisation of a number
    #[command(arg_required_else_help = true)]
    PrimeFactors {
        #[arg(short = 'n', long = "num", value_name = "NUMBER")]
        num: BigInt,
    },

    /// Find the prime factorisation of a range of numbers
    #[command(arg_required_else_help = true)]
    PrimeFactorsRange(PFactorsArgs),

    /// Primality checking
    #[command(arg_required_else_help = true)]
    Primality(PrimalityArgs),
    
    /// Carmichael Number search
    #[command(arg_required_else_help = true)]
    CarmichaelNums(CarmichaelNumsArgs),


    // Check if a number is Prime or Composite using Miller Rabin
    #[command(arg_required_else_help = true)]
    MillerRabin {
        #[arg(short = 'n', long = "num", value_name = "NUMBER")]
        num: BigInt,
    },

    /// Ass1 - Question3
    #[command(arg_required_else_help = true)]
    Ass1Question3(NumRangeArgs),

    /// Ass1 - Question3
    #[command(arg_required_else_help = true)]
    AKS(NumRangeArgs),

    // 'R' value calculation for AKS
    #[command(arg_required_else_help = true)]
    FindrAKS {
        #[arg(short = 'n', long = "num", value_name = "NUMBER")]
        num: BigInt,
    },

    /// Modular Exponentiation Calculator
    #[command(arg_required_else_help = true)]
    ModularPower {
        #[arg(short = 'b', long = "base", value_name = "BASE NUMBER")]
        base: BigInt,

        #[arg(short = 'e', long = "exponent", value_name = "Exponent Value")]
        exponent: BigInt,

        #[arg(short = 'm', long = "modulus", value_name = "Modulus Value")]
        modulus: BigInt,
    },
    #[command(arg_required_else_help = true)]
    PollardsRhoLog {
        #[arg(short = 'a', value_name = "primitive root mod n")]
        a: BigInt,

        #[arg(short = 'b', value_name = "calculating logarithm of b")]
        b: BigInt,

        #[arg(short = 'n', value_name = "Modulus Value")]
        n: BigInt,
    },
    PrimitiveRoots(PrimitiveRootsArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct PFactorsArgs {
    #[command(subcommand)]
    pub command: PFactorsCommands,
}

#[derive(Debug, Subcommand)]
pub enum PFactorsCommands {
    All(NumRangeArgs),
    Composites(NumRangeArgs),
    CompositesPQ(NumRangeArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CarmichaelNumsArgs {
    #[command(subcommand)]
    pub command: CarmichaelNumsCommands,
}

#[derive(Debug, Subcommand)]
pub enum CarmichaelNumsCommands {
    Korselt(NumRangeArgs),
    FermatLT(NumRangeArgs),
}

#[derive(Debug, Args, Clone)]
pub struct NumRangeArgs {
    #[arg(
        short = 's',
        long = "start",
        value_name = "START NUMBER for a range of numbers"
    )]
    pub start: BigInt,

    #[arg(short = 'e', long = "end", value_name = "END NUMBER for the range")]
    pub end: BigInt,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Table,
    Json,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct PrimalityArgs {
    #[command(subcommand)]
    pub command: PrimalityCommands,
}

#[derive(Debug, Subcommand)]
pub enum PrimalityCommands {
    GCD(PrimalityRangeArgs),
}

#[derive(Debug, Args)]
pub struct PrimalityRangeArgs {
    #[arg(
        short,
        long,
        value_name = "This must be present if primality check is for a single number"
    )]
    pub num: Option<BigInt>,

    #[arg(
        short = 's',
        long = "start",
        value_name = "START NUMBER for a range of numbers"
    )]
    pub start: Option<BigInt>,

    #[arg(short = 'e', long = "end", value_name = "END NUMBER for the range")]
    pub end: Option<BigInt>,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct PrimitiveRootsArgs {
    #[command(subcommand)]
    pub command: PrimitiveRootsCommands,
}

#[derive(Debug, Subcommand)]
pub enum PrimitiveRootsCommands {
    /// Search for numbers with primitive roots in a range
    SearchNumsWithPrimitiveRoots(NumRangeArgs),
    /// List the primitive roots of a number
    #[command(arg_required_else_help = true)]
    ListPrimitiveRoots {
        /// Integer
        n: BigInt,
    },
    /// Assignment 2 - Question 2(b)
    Ass2Question2b(NumRangeArgs),
    
    /// Assignment 2 - Question 2(c)
    Ass2Question2c(NumRangeArgs),

    /// Assignment 2 - Question 3(d)
    Ass2Question3d(NumRangeArgs)
}