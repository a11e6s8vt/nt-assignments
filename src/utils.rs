use std::marker::PhantomPinned;

use num_bigint::{BigInt, BigUint};
use num_iter::{range, range_inclusive};
use num_traits::{One, Zero};
use num_traits::{Pow, ToPrimitive};
use rand::seq::index;
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

pub fn abs_log(x: &BigInt) -> Result<f64, String> {
    use std::cmp::Ordering;
    let zero = BigInt::zero();
    let x: BigUint = match x.cmp(&zero) {
        Ordering::Less => (-x).to_biguint().unwrap(),
        Ordering::Greater => x.to_biguint().unwrap(),
        Ordering::Equal => return Err("abs_log(0)".to_string()),
    };
    let x: Vec<u8> = x.to_bytes_le();
    use num_traits::Float;
    const BYTES: usize = 12;
    let start = if x.len() < BYTES { 0 } else { x.len() - BYTES };
    let mut n: f64 = 0.0;
    // n will accumulate the f64 value as we go.
    for i in start..x.len() {
        n = n / 256.0 + (x[i] as f64);
    }
    let ln_256: f64 = (256.0).ln();
    Ok(n.ln() + ln_256 * ((x.len() - 1) as f64))
}

///
/// Use fast modular exponentiation for polynomials to raise them to a big power.
///
pub fn fastpoly(base: &Vec<BigInt>, power: &BigInt, r: &BigInt) -> Vec<BigInt> {
    let mut base = base.clone();
    let mut x = Vec::<BigInt>::new();
    let a = &base[0].clone();

    for i in 0..base.len() {
        x.push(BigInt::zero());
    }
    x[0] = BigInt::one();
    let n = power.clone();
    let mut power = power.clone();

    while &power > &BigInt::zero() {
        if &power % &BigInt::from(2u64) == BigInt::one() {
            x = polynomial_mul(&x, &base, &n, &r);
        }
        base = polynomial_mul(&base, &base, &n, &r);
        power = power / BigInt::from(2u64);
    }
    x[0] = &x[0] - a;
    let index = &n % r;
    let index = index.to_usize().unwrap_or(usize::MAX);
    x[index] = &x[index] - BigInt::one();
    x
}

///Function used by fastPoly to multiply two polynomials together.
pub fn polynomial_mul(a: &Vec<BigInt>, b: &Vec<BigInt>, n: &BigInt, r: &BigInt) -> Vec<BigInt> {
    let mut x = Vec::<BigInt>::new();
    for i in 0..a.len() + b.len() - 1 {
        x.push(BigInt::zero());
    }
    for i in 0..a.len() {
        for j in 0..b.len() {
            // usize::MAX is not ideal here. I couldn't find a way to index the vec
            let index = (i + j) % r.to_usize().unwrap_or(usize::MAX);
            x[index] += a[i].clone() * b[j].clone();
            x[index] = &x[index] % n.clone();
        }
    }
    for _ in range(r.clone(), BigInt::from(x.len())) {
        x.pop();
    }
    return x;
}

///
/// Find smallest r such that the order of n mod r > ln(n)^2.
///
pub fn findr(n: &BigInt) -> BigInt {
    let (zero, one) = (BigInt::zero(), BigInt::one());
    let mut r = BigInt::from(1u64);

    let s: f64 = abs_log(n).unwrap().pow(2);
    let s = BigInt::from(s.floor() as u64);
    let mut nex_r = true;

    while nex_r {
        r += 1;
        nex_r = false;
        let mut k = BigInt::zero();
        while &k <= &s && nex_r == false {
            k += 1;
            if modular_pow(n, &k, &r) == zero || modular_pow(n, &k, &r) == one {
                nex_r = true;
            }
        }
    }

    r
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
    fn test_abs_log() {
        assert_eq!(2.995732273553991, abs_log(&BigInt::from(20u64)).unwrap());
    }
}
