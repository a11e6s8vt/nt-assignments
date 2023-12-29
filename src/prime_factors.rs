use std::arch::x86_64;
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::primality::miller_rabin_primality;
use dpc_pariter::IteratorExt;
use itertools::Itertools;
use num_bigint::BigInt;
use num_iter::{range, range_inclusive, Range, RangeInclusive};
use num_traits::identities::One;
use num_traits::Zero;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelExtend};
use rayon::vec;

pub trait PrimeFactors {
    fn prime_factors(&self, primes: &mut Vec<BigInt>) -> Vec<(BigInt, usize)>;
    fn is_prime_factors_form_pq(&self) -> (bool, Vec<(BigInt, usize)>);
}

impl PrimeFactors for BigInt {
    fn prime_factors(&self, primes: &mut Vec<BigInt>) -> Vec<(Self, usize)> {
        let mut n = self.clone();
        // Check if n is prime
        if miller_rabin_primality(&self) {
            return vec![(self.clone(), 1)];
        }

        let start_no = primes.last().unwrap();
        let square_root = self.sqrt();
        if square_root - start_no > BigInt::from(2u64) {
            let end_no: BigInt = self.sqrt() + 1; // +1 to get the ceiling value
                                                  // println!("start = {}, end = {}", start_no, end_no);

            let r = range(start_no.clone(), end_no);

            // let mut primes: Vec<BigInt> = Vec::new();

            // for m in r {
            //     if miller_rabin_primality(&m) {
            //         primes.push(m);
            //     }
            // }
            let new_primes: Vec<BigInt> = r
                .into_iter()
                .map(|x| x)
                .parallel_filter(|x| miller_rabin_primality(x))
                .collect();
            // println!("{:?}", new_primes);
            primes.extend(new_primes);
            let mut seen = HashSet::new();
            primes.retain(|c| seen.insert(c.clone()));
            // println!("{:?}", primes);
        }
        let mut res: HashMap<BigInt, usize> = HashMap::new();

        // 'outer: while n > BigInt::one() {
        //     for p in primes.iter() {
        //         println!("p = {}", p);
        //         if &n % p == BigInt::zero() {
        //             res.entry(p.clone()).and_modify(|c| *c += 1).or_insert(1);
        //             n = n / p;
        //             if miller_rabin_primality(&n) {
        //                 res.entry(n).and_modify(|c| *c += 1).or_insert(1);
        //                 break 'outer;
        //             }
        //             break;
        //         } else {
        //             continue;
        //         }
        //     }
        // }

        // all_divisors will contain all the divisors of num with repetition.
        // The product of the elements of all_divisors will equal the "num"
        let mut all_divisors = Vec::<BigInt>::new(); //
        let mut product = BigInt::one();

        while product < n {
            let divisors = primes
                .par_iter()
                .filter(|x| (n.clone() / &product) % *x == BigInt::zero())
                .map(|p| p.clone())
                .collect::<Vec<BigInt>>();
            all_divisors.extend(divisors.clone());
            // println!("{:?}", res);
            product = product
                * divisors
                    .iter()
                    .fold(BigInt::one(), |acc: BigInt, a| acc * a);
            let q = &n / &product;
            if miller_rabin_primality(&q) {
                all_divisors.push(q);
                break;
            }
        }

        // println!("n = {}, res = {:?}", n, res);
        let mut res = all_divisors
            .into_iter()
            .fold(HashMap::<BigInt, usize>::new(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            })
            .into_iter()
            .filter_map(|(k, v)| Some((k, v)))
            .collect::<Vec<(BigInt, usize)>>();
        res.sort_by_key(|k| k.0.clone());
        res
    }

    fn is_prime_factors_form_pq(&self) -> (bool, Vec<(BigInt, usize)>) {
        let mut primes = vec![BigInt::from(2u64)];
        let p_factors = self.prime_factors(&mut primes);
        if p_factors.len() != 2 {
            return (false, vec![]);
        }

        let first = p_factors.first().unwrap();
        let second = p_factors.get(1).unwrap();

        match first.1 {
            1 => match second.1 {
                1 => (true, p_factors),
                _ => (false, vec![]),
            },
            _ => (false, vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors() {
        let b1 = BigInt::from(100u64);
        let mut primes = vec![BigInt::from(2u64)];
        let result = b1.prime_factors(&mut primes);
        assert_eq!(
            result,
            vec![(BigInt::from(2u64), 2), (BigInt::from(5u64), 2)]
        );
    }

    // #[test]
    // fn test_is_form_pq_prime_factors() {
    // let result = prime_factors(100);
    // assert_eq!(result, vec![(2, 2), (5, 2)]);
    // }
    // #[test]
    // fn test_primes_less_than_n() {
    //     let test_sample = vec![
    //         BigInt::from(2u64),
    //         BigInt::from(3u64),
    //         BigInt::from(5u64),
    //         BigInt::from(7u64),
    //         BigInt::from(11u64),
    //         BigInt::from(13u64),
    //         BigInt::from(17u64),
    //         BigInt::from(19u64),
    //     ];

    //     assert_eq!(test_sample, primes_less_than_n(&BigInt::from(20u64)));
    // }
}
