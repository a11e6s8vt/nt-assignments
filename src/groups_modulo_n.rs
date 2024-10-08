use core::num;

use num_bigint::BigInt;
use num_iter::range;
use num_traits::{One, Zero};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    prime_factors::PrimeFactors,
    utils::{modular_pow, Gcd},
};

///
/// Generates a list of integers less than n and co-prime to n.
///
pub fn coprime_nums_less_than_n(n: &BigInt) -> Vec<BigInt> {
    let mut coprimes: Vec<BigInt> = Vec::new();
    let r = range(BigInt::from(1u64), n.clone());

    for num in r {
        if n.gcd_euclid(&num) == BigInt::one() {
            coprimes.push(num)
        }
    }
    coprimes.sort();
    coprimes
}

///
/// Get list of divisors of a number n > 2
///
pub fn divisors_of_n(n: &BigInt) -> Vec<BigInt> {
    let mut divisors: Vec<BigInt> = Vec::new();
    let mut primes = vec![BigInt::from(2u64)];
    let p_factors_n = n.prime_factors(&mut primes);
    let p_factors_n = p_factors_n
        .iter()
        .map(|(p, _)| p.clone())
        .collect::<Vec<BigInt>>();

    for p in p_factors_n {
        let mut i = 0;
        loop {
            let pow = p.pow(i);
            if n % &pow == BigInt::zero() {
                divisors.push(n / &pow);
                divisors.push(pow);
                i += 1;
            } else {
                break;
            }
        }
    }
    divisors.sort();
    divisors.dedup();
    divisors
}

///
/// `euler_totient_phi_v1` calculates the phi value by counting the coprimes to n
///
pub fn euler_totient_phi_counting_coprimes(n: &BigInt) -> BigInt {
    let coprimes = coprime_nums_less_than_n(n);
    BigInt::from(coprimes.len())
}

///
/// `euler_totient_phi` calculates the phi value using prime factorisation
///
pub fn euler_totient_phi(n: &BigInt) -> BigInt {
    let mut primes = vec![BigInt::from(2u64)];
    let p_factors = n.prime_factors(&mut primes);
    let phi: BigInt = p_factors
        .iter()
        .map(|(p, a)| (p - 1) * p.pow(*a as u32 - 1))
        .product();
    phi
}

///
/// Returns a vec of primitive roots for the integer
///
/// # Arguments
/// * n: BigInt
///
/// Steps:
/// This function uses trial and error to find primitive roots associated to an Integer
///
/// 1. Find all coprime numbers less than `n`
/// 2. ϕ(n) = total number of coprimes
/// 3. Find all the divisors of ϕ(n). Order of an element in the Modulo n group
///    will be equal to any of the divisor values.
/// 4. Find the order of each of the coprimes to n one by one (skip 1 from the list of
///    coprimes as 1 is a trivial root) (`use utils::modular_pow)
/// 5. if order of a coprime integer equals ϕ(n), that coprime is a primitive root
///
/// The above steps are executed aginst all coprimes to n and returns an integer vector with
/// primitive roots
///
pub fn primitive_roots_trial_n_error(n: &BigInt) -> Vec<BigInt> {
    let mut primitive_roots: Vec<BigInt> = Vec::new();
    let mut has_primitive_roots: bool = false;

    let nums_coprime_n: Vec<BigInt> = coprime_nums_less_than_n(n);
    let phi_n = BigInt::from(nums_coprime_n.len());
    //
    let divisors_phi_n = divisors_of_n(&phi_n);

    for a in nums_coprime_n {
        let mut has_order_phi: bool = true;
        for order in divisors_phi_n.iter() {
            if modular_pow(&a, order, n) == BigInt::one() {
                if *order != phi_n {
                    has_order_phi = false;
                }
            }
        }

        if has_order_phi {
            primitive_roots.push(a);
            has_primitive_roots = true;
            break;
        }
    }

    if has_primitive_roots {
        let orders_coprime_phi_n: Vec<BigInt> = coprime_nums_less_than_n(&phi_n);
        // first coprime number is 1 and we are skipping that when calculating power
        for order in orders_coprime_phi_n.iter().skip(1) {
            primitive_roots.push(modular_pow(&primitive_roots[0], order, n));
        }
    }

    primitive_roots.sort();

    for (i, num) in primitive_roots.clone().iter().enumerate() {
        if num == &BigInt::one() {
            primitive_roots.remove(i);
            continue;
        }

        if modular_pow(num, &phi_n, n) != BigInt::one() {
            primitive_roots.remove(i);
        }
    }

    primitive_roots
}

/// It checks the existence of primitive roots modulo n
/// and returns the number of primitive roots
pub fn primitive_roots_count_modulo_n(n: &BigInt) -> BigInt {
    let (zero, two) = (BigInt::zero(), BigInt::from(2u64));
    let mut primes = vec![BigInt::from(2u64)];
    let mut p_factors = n.prime_factors(&mut primes);
    if p_factors.len() < 1 || p_factors.len() > 2 {
        return BigInt::zero();
    }

    match p_factors.len() {
        1 => {
            if let Some(first) = p_factors.pop() {
                if first.0 == two {
                    if first.1 < 1 || first.1 > 2 {
                        return zero;
                    }
                }
            }
        }
        2 => {
            let first = p_factors.remove(0);
            if first.0 == two {
                if first.1 > 1 {
                    return zero;
                }
            } else {
                return zero;
            }
        }
        _ => return zero,
    }
    let phi_n: BigInt = euler_totient_phi(n);
    let phi_phi_n = euler_totient_phi(&phi_n);
    phi_phi_n
}

/// It checks the existence of primitive roots modulo n
/// and returns the number of primitive roots
pub fn is_integer_of_form_pk_2pk(n: &BigInt) -> Vec<(BigInt, usize)> {
    let (_zero, two) = (BigInt::zero(), BigInt::from(2u64));
    let mut primes = vec![BigInt::from(2u64)];
    let p_factors = n.prime_factors(&mut primes);
    if p_factors.len() < 1 || p_factors.len() > 2 {
        return vec![];
    }

    // p_factors is a sorted vector
    // check the criteria on on a clone of p_factors
    let mut p_factors_clone = p_factors.clone();
    match p_factors_clone.len() {
        // If the prime factors have a length of 1,
        // then it must be of the form p^k. Hence, if
        // p = 2, fail
        1 => {
            if let Some(first) = p_factors_clone.pop() {
                if &first.0 == &two {
                    return vec![];
                }
            }
        }
        // If the prime factors have a length of 2,
        // then the first factor is 2^1 and the second factor is p^k
        2 => {
            let first = p_factors_clone.remove(0);
            if &first.0 == &two {
                // since the p_factors vec is prepared in sorted form and without
                // duplicates, we only need to check the first
                if first.1 > 1 {
                    return vec![];
                }
            } else {
                return vec![];
            }
        }
        _ => return vec![],
    }

    p_factors
}

///
///
///
pub fn multiplicative_order(a: &BigInt, n: &BigInt) -> Option<BigInt> {
    if n.gcd_euclid(a) != BigInt::one() {
        // return zero as the numbers are not coprime
        return None;
    }

    let phi_n = euler_totient_phi_counting_coprimes(n);
    let divisors_phi_n = divisors_of_n(&phi_n);
    let all_possible_orders_a = divisors_phi_n
        .par_iter()
        .filter(|x| modular_pow(a, x, n) == BigInt::one())
        .map(|x| x.clone())
        .collect::<Vec<BigInt>>();
    // let p_factors = n.prime_factors(&mut primes);
    all_possible_orders_a.into_iter().min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors_of_n() {
        let result = divisors_of_n(&BigInt::from(160u64));
        let d: Vec<BigInt> = vec![
            BigInt::from(1u64),
            BigInt::from(2u64),
            BigInt::from(4u64),
            BigInt::from(5u64),
            BigInt::from(8u64),
            BigInt::from(10u64),
            BigInt::from(16u64),
            BigInt::from(20u64),
            BigInt::from(32u64),
            BigInt::from(40u64),
            BigInt::from(80u64),
            BigInt::from(160u64),
        ];
        assert_eq!(result, d);
    }

    #[test]
    fn test_get_integers_coprime_n_1() {
        let result = coprime_nums_less_than_n(&BigInt::from(10u64));
        assert_eq!(
            result,
            vec![
                BigInt::from(1u64),
                BigInt::from(3u64),
                BigInt::from(7u64),
                BigInt::from(9u64)
            ]
        );
    }

    #[test]
    fn test_get_integers_coprime_n_2() {
        let result = coprime_nums_less_than_n(&BigInt::from(17u64));
        let s = range(BigInt::from(1u64), BigInt::from(17u64))
            .map(|x| x.clone())
            .collect::<Vec<BigInt>>();
        assert_eq!(result, s);
    }

    #[test]
    fn test_euler_totient_phi_v1() {
        let result = euler_totient_phi_counting_coprimes(&BigInt::from(378u64));
        assert_eq!(result, BigInt::from(108u64));

        let result = euler_totient_phi_counting_coprimes(&BigInt::from(601u64));
        assert_eq!(result, BigInt::from(600u64));
    }

    #[test]
    fn test_euler_totient() {
        let result = euler_totient_phi(&BigInt::from(378u64));
        assert_eq!(result, BigInt::from(108u64));
    }

    #[test]
    fn test_primitive_roots_trial_n_error() {
        let result = primitive_roots_trial_n_error(&BigInt::from(25u64));
        assert_eq!(
            result,
            vec![
                BigInt::from(2u64),
                BigInt::from(3u64),
                BigInt::from(8u64),
                BigInt::from(12u64),
                BigInt::from(13u64),
                BigInt::from(17u64),
                BigInt::from(22u64),
                BigInt::from(23u64)
            ]
        )
    }

    #[test]
    fn test_primitive_roots_count_modulo_n() {
        let result = primitive_roots_count_modulo_n(&BigInt::from(1250u64));
        assert_eq!(result, BigInt::from(200u64));
        let result = primitive_roots_count_modulo_n(&BigInt::from(59u64));
        assert_eq!(result, BigInt::from(28u64));
        let result = primitive_roots_count_modulo_n(&BigInt::from(20u64));
        assert_eq!(result, BigInt::from(0u64));
        let result = primitive_roots_count_modulo_n(&BigInt::from(30u64));
        assert_eq!(result, BigInt::from(0u64));
        let result = primitive_roots_count_modulo_n(&BigInt::from(10u64));
        assert_eq!(result, BigInt::from(2u64));
        let result = primitive_roots_count_modulo_n(&BigInt::from(40u64));
        assert_eq!(result, BigInt::from(0u64));
    }

    #[test]
    fn test_multiplicative_order() {
        assert_eq!(
            BigInt::from(50u64),
            multiplicative_order(&BigInt::from(45u64), &BigInt::from(101u64)).unwrap()
        );
    }

    #[test]
    fn test_multiplicative_order_none() {
        assert_eq!(
            None,
            multiplicative_order(&BigInt::from(45u64), &BigInt::from(100u64))
        );
    }

    #[test]
    fn test_is_integer_of_form_pk_2pk() {
        let result = is_integer_of_form_pk_2pk(&BigInt::from(25u64));
        assert_eq!(result, vec![(BigInt::from(5u64), 2usize)]);

        let result = is_integer_of_form_pk_2pk(&BigInt::from(20u64));
        assert_eq!(result, vec![]);

        let result = is_integer_of_form_pk_2pk(&BigInt::from(147u64));
        assert_eq!(result, vec![]);

        let result = is_integer_of_form_pk_2pk(&BigInt::from(49u64));
        assert_eq!(result, vec![(BigInt::from(7u64), 2usize)]);
    }
}
