use std::arch::x86_64;

use crate::primality::{is_prime_trial_division, is_prime_trial_division_parallel};
use itertools::Itertools;
use num_bigint::BigInt;
use num_iter::range_inclusive;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub fn find_primes_in_range_trial_division(start: BigInt, end: BigInt) -> Vec<BigInt> {
    let mut nums_categorised = range_inclusive(start, end)
        .par_bridge()
        .into_par_iter()
        .map(|x| (x.clone(), is_prime_trial_division_parallel(&x)))
        .collect::<Vec<(BigInt, bool)>>();
    // let mut categorised = range_inclusive(start, end)
    //     .into_iter()
    //     .map(|x| (x.clone(), is_prime_trial_division(&x)))
    //     .collect::<Vec<(BigInt, bool)>>().sort();
    let mut primes: Vec<BigInt> = Vec::new();
    let mut composites: Vec<BigInt> = Vec::new();
    for x in nums_categorised {
        if x.1 == true {
            primes.push(x.0)
        } else {
            composites.push(x.0)
        }
    }
    primes.sort();
    composites.sort();
    println!("{:?}", primes);
    println!("{:?}", composites);
    Vec::<BigInt>::new()
}
