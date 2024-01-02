use std::collections::{HashMap, HashSet};

use crate::primality::miller_rabin_primality;
use dpc_pariter::IteratorExt;
use num_bigint::BigInt;
use num_iter::range_inclusive;
use num_traits::identities::One;
use num_traits::Zero;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub trait PrimeFactors {
    fn prime_factors(&self, primes: &mut Vec<BigInt>) -> Vec<(BigInt, usize)>;
    //fn is_prime_factors_form_pq(&self) -> (bool, Vec<(BigInt, usize)>);
}

impl PrimeFactors for BigInt {
    fn prime_factors(&self, primes: &mut Vec<BigInt>) -> Vec<(Self, usize)> {
        let n = self.clone();
        // Check if n is prime
        if miller_rabin_primality(&self) {
            return vec![(self.clone(), 1)];
        }

        let start_no = primes.last().unwrap();
        let square_root = self.sqrt();
        if square_root - start_no > BigInt::from(2u64) {
            let end_no: BigInt = self.sqrt() + 1; // +1 to get the ceiling value
                                                  // println!("start = {}, end = {}", start_no, end_no);

            let r = range_inclusive(start_no.clone(), end_no);

            let new_primes: Vec<BigInt> = r
                .into_iter()
                .map(|x| x)
                .parallel_filter(|x| miller_rabin_primality(x))
                .collect();
            primes.extend(new_primes);
            let mut seen = HashSet::new();
            primes.retain(|c| seen.insert(c.clone()));
        }
        let _res: HashMap<BigInt, usize> = HashMap::new();

        // The all_divisors vec will contain all the divisors of num with repetition.
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
}
