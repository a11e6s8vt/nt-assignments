use num_bigint::BigInt;
use num_iter::range_inclusive;
use num_traits::{One, Zero};
use rand::Rng;

use crate::prime_factors::PrimeFactors;

pub trait Gcd {
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::Gcd;
    ///
    /// assert_eq!(BigInt::from(44u64), BigInt::from(2024u64).gcd_euclid(&BigInt::from(748u64)));
    /// ```

    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
    fn gcd_euclid(&self, other: &Self) -> Self;
}

impl Gcd for BigInt {
    ///
    /// GCD Calculator - The Euclidean Algorithm
    /// Input: A pair of integers a and b, not both equal to zero
    /// Output: gcd(a, b)
    ///
    fn gcd_euclid(&self, other: &BigInt) -> BigInt {
        let zero = BigInt::from(0u64);
        let mut a = self.clone();
        let mut b = other.clone();
        let mut gcd: BigInt = zero.clone();
        if b > a {
            gcd = b.gcd_euclid(&a);
        } else {
            let mut r: BigInt = &a % &b;
            while &r > &zero {
                // let q = &a / &b;
                r = &a % &b;

                if &r != &zero {
                    a = b;
                    b = r.clone();
                }
            }

            gcd = b;
        }

        gcd
    }
}

///
/// Returns a non-negative integer a < m that satisfies a ≡ cˣ(mod m)
/// c: base
/// e: exponent
/// m: modulus
///
pub fn modular_pow(base: &BigInt, e: &BigInt, modulus: &BigInt) -> BigInt {
    // initialization
    let (zero, one, two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let mut exp = e.clone();
    let mut a: BigInt = BigInt::from(1u64);
    let mut s: BigInt = base % modulus;

    // Converts exponent to its binary representation
    // Go through the digits from LSB to MSB in each iteration
    // if the digit == 1, a = a * s % modulus, s = s * s
    // if digit == 0, s = s * s
    while exp > zero {
        // Extract the LSB from the exp.
        if &exp & &one == one {
            a = (a * &s) % modulus;
        }

        s = (&s * &s) % modulus;

        // Division by 2 to get the next digit
        exp = exp / &two;
    }

    a
}

///
/// Generate a random integer in a given range
///
pub fn generate_random_int_in_range(a: &BigInt, b: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    // return a random BigInt between a and b
    rng.gen_range(a.clone()..b.clone())
}

///
/// Generates a list of integers less than n and co-prime to n.
///
pub fn coprime_nums_less_than_n(n: &BigInt) -> Vec<BigInt> {
    let mut coprimes: Vec<BigInt> = Vec::new();
    let r = range_inclusive(BigInt::from(2u64), n.clone());

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
pub fn euler_totient_phi_v1(n: &BigInt) -> BigInt {
    let coprimes = coprime_nums_less_than_n(n);
    println!("{:?}", coprimes);
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
/// To find all primitive roots modulo n, we follow these steps:
///
pub fn primitive_roots_trial_n_error(n: &BigInt) -> Vec<BigInt> {
    let mut primitive_roots: Vec<BigInt> = Vec::new();
    let mut has_primitive_roots: bool = false;
    let phi_n = euler_totient_phi(n);
    //
    let divisors_phi_n = divisors_of_n(&phi_n);
    let nums_coprime_n: Vec<BigInt> = coprime_nums_less_than_n(n);

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
/// an returns the number of primitive roots
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
                match first.0 {
                    two => {
                        if first.1 < 1 || first.1 > 2 {
                            return zero;
                        }
                    }
                    _ => {}
                }
            }
        }
        2 => {
            let first = p_factors.remove(0);
            match first.0 {
                two => {
                    if first.1 > 1 {
                        return zero;
                    }
                }
                _ => return zero,
            }
        }
        _ => return zero,
    }
    let phi_n: BigInt = euler_totient_phi(n);
    let phi_phi_n = euler_totient_phi(&phi_n);
    phi_phi_n
}

#[cfg(test)]
mod tests {
    use num_iter::range;

    use super::*;

    #[test]
    fn test_gcd_euclid_1() {
        let a = BigInt::from(100u64);
        let result = a.gcd_euclid(&BigInt::from(76u64));
        assert_eq!(result, BigInt::from(4u64));
        assert_eq!(
            BigInt::from(44u64),
            BigInt::from(2024u64).gcd_euclid(&BigInt::from(748u64))
        );
    }

    #[test]
    fn test_modular_pow() {
        let result = modular_pow(
            &BigInt::from(2u64),
            &BigInt::from(825u64),
            &BigInt::from(173u64),
        );
        assert_eq!(result, BigInt::from(107u64));
    }

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
            vec![BigInt::from(3u64), BigInt::from(7u64), BigInt::from(9u64)]
        );
    }

    #[test]
    fn test_get_integers_coprime_n_2() {
        let result = coprime_nums_less_than_n(&BigInt::from(17u64));
        let s = range(BigInt::from(2u64), BigInt::from(17u64))
            .map(|x| x.clone())
            .collect::<Vec<BigInt>>();
        assert_eq!(result, s);
    }

    #[test]
    fn test_euler_totient_phi_v1() {
        let result = euler_totient_phi_v1(&BigInt::from(378u64));
        assert_eq!(result, BigInt::from(108u64));

        let result = euler_totient_phi_v1(&BigInt::from(601u64));
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
}
