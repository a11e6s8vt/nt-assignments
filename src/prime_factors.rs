use std::collections::HashMap;

use crate::primality::miller_rabin_primality;
use num_bigint::BigInt;
use num_iter::{range, range_inclusive, Range, RangeInclusive};
use num_traits::identities::One;
use num_traits::Zero;
use rayon::iter::ParallelExtend;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub trait PrimeFactors {
    fn prime_factors(&self) -> Vec<(BigInt, usize)>;
    fn is_form_pq_prime_factors(&self) -> (bool, Vec<(BigInt, usize)>);
}

impl PrimeFactors for BigInt {
    fn prime_factors(&self) -> Vec<(Self, usize)> {
        let mut n = self.clone();
        // Check if n is prime
        if miller_rabin_primality(&self) {
            return vec![(self.clone(), 1)];
        }

        let start_no = BigInt::from(2u64);
        let mut end_no: BigInt = self.sqrt();
        end_no += 1; // +1 to get the ceiling value

        let r = range(start_no, end_no);

        let mut primes: Vec<BigInt> = Vec::new();

        for m in r {
            if miller_rabin_primality(&m) {
                primes.push(m);
            }
        }

        let mut res: HashMap<BigInt, usize> = HashMap::new();

        'outer: while n > BigInt::one() {
            for p in primes.iter() {
                if &n % p == BigInt::zero() {
                    res.entry(p.clone()).and_modify(|c| *c += 1).or_insert(1);
                    n = n / p;
                    if miller_rabin_primality(&n) {
                        res.entry(n).and_modify(|c| *c += 1).or_insert(1);
                        break 'outer;
                    }
                    break;
                } else {
                    continue;
                }
            }
        }

        let mut res = res
            .into_iter()
            .filter_map(|(key, value)| Some((key, value)))
            .collect::<Vec<(BigInt, usize)>>();
        res.sort_by_key(|k| k.0.clone());
        res
    }

    fn is_form_pq_prime_factors(&self) -> (bool, Vec<(BigInt, usize)>) {
        (true, vec![])
    }
}

// fn primes_less_than_n(n: &BigInt) -> Vec<BigInt> {
//     let mut primes = vec![BigInt::from(2u64)];
//     let mut new_primes = Vec::<BigInt>::new();
//     let last_prime = primes.last().unwrap();

//     new_primes.par_extend(
//         (range(last_prime.clone(), n.clone()))
//             .par_bridge()
//             .into_par_iter()
//             .filter(|candidate| {
//                 let square_root = candidate.sqrt() + 1;
//                 println!("can = {}", &candidate);
//                 primes
//                     .iter()
//                     .take_while(|p| p <= &&square_root)
//                     .all(|p| candidate % p != BigInt::zero())
//             }),
//     );
//     primes.extend(new_primes);
//     primes.dedup();

//     primes
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors() {
        let b1 = BigInt::from(100u64);
        let result = b1.prime_factors();
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
