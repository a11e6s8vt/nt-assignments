use std::arch::x86_64;

use crate::primality::{is_prime_trial_division, is_prime_trial_division_parallel};
use itertools::Itertools;
use num_bigint::BigInt;
use num_iter::range_inclusive;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub fn find_primes_in_range_trial_division_parallel(
    start: BigInt,
    end: BigInt,
) -> (Vec<BigInt>, Vec<BigInt>) {
    let nums_categorised = range_inclusive(start, end)
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
    (primes, composites)
}

pub fn find_primes_in_range_trial_division(
    start: BigInt,
    end: BigInt,
) -> (Vec<BigInt>, Vec<BigInt>) {
    let nums_categorised = range_inclusive(start, end)
        .into_iter()
        .map(|x| (x.clone(), is_prime_trial_division(&x)))
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
    (primes, composites)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> (BigInt, BigInt, Vec<BigInt>) {
        let start = BigInt::from(2800u64);
        let end = BigInt::from(3100u64);

        let primes_in_range = vec![
            BigInt::from(2801u64),
            BigInt::from(2803u64),
            BigInt::from(2819u64),
            BigInt::from(2833u64),
            BigInt::from(2837u64),
            BigInt::from(2843u64),
            BigInt::from(2851u64),
            BigInt::from(2857u64),
            BigInt::from(2861u64),
            BigInt::from(2879u64),
            BigInt::from(2887u64),
            BigInt::from(2897u64),
            BigInt::from(2903u64),
            BigInt::from(2909u64),
            BigInt::from(2917u64),
            BigInt::from(2927u64),
            BigInt::from(2939u64),
            BigInt::from(2953u64),
            BigInt::from(2957u64),
            BigInt::from(2963u64),
            BigInt::from(2969u64),
            BigInt::from(2971u64),
            BigInt::from(2999u64),
            BigInt::from(3001u64),
            BigInt::from(3011u64),
            BigInt::from(3019u64),
            BigInt::from(3023u64),
            BigInt::from(3037u64),
            BigInt::from(3041u64),
            BigInt::from(3049u64),
            BigInt::from(3061u64),
            BigInt::from(3067u64),
            BigInt::from(3079u64),
            BigInt::from(3083u64),
            BigInt::from(3089u64),
        ];
        (start, end, primes_in_range)
    }

    #[test]
    fn test_find_primes_in_range_trial_division_parallel() {
        let (start, end, primes_in_range) = test_data();
        assert_eq!(
            primes_in_range,
            find_primes_in_range_trial_division_parallel(start, end).0
        );
    }

    #[test]
    fn test_find_primes_in_range_trial_division() {
        let (start, end, primes_in_range) = test_data();
        assert_eq!(
            primes_in_range,
            find_primes_in_range_trial_division(start, end).0
        );
    }
}
