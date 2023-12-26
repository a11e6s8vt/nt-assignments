use num_bigint::BigInt;
<<<<<<< HEAD
<<<<<<< HEAD
use num_iter::range_inclusive;
use num_traits::One;
use rand::Rng;

=======
>>>>>>> 5407e32 (prime factors - tabular print)
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
<<<<<<< HEAD
=======
>>>>>>> 780bc8c (miller-rabin)
=======
>>>>>>> 5407e32 (prime factors - tabular print)

///
/// Returns a non-negative integer a < m that satisfies a ≡ cˣ(mod m)
/// c: base
/// e: exponent
/// m: modulus
///
<<<<<<< HEAD
pub fn modular_pow(base: &BigInt, e: &BigInt, modulus: &BigInt) -> BigInt {
=======
pub fn modular_pow(c: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
>>>>>>> 780bc8c (miller-rabin)
    // initialization
    let (zero, one, two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let mut exp = e.clone();
    let mut a: BigInt = BigInt::from(1u64);
<<<<<<< HEAD
    let mut s: BigInt = base % modulus;
=======
    let mut s: BigInt = c % m;
>>>>>>> 780bc8c (miller-rabin)

    // Converts exponent to its binary representation
    // Go through the digits from LSB to MSB in each iteration
    // if the digit == 1, a = a * s % modulus, s = s * s
    // if digit == 0, s = s * s
    while exp > zero {
        // Extract the LSB from the exp.
        if &exp & &one == one {
<<<<<<< HEAD
            a = (a * &s) % modulus;
        }

        s = (&s * &s) % modulus;
=======
            a = (a * &s) % m;
        }

        s = (&s * &s) % m;
>>>>>>> 780bc8c (miller-rabin)

        // Division by 2 to get the next digit
        exp = exp / &two;
    }

    a
}

<<<<<<< HEAD
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

=======
>>>>>>> 780bc8c (miller-rabin)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 5407e32 (prime factors - tabular print)
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
<<<<<<< HEAD
=======
>>>>>>> 780bc8c (miller-rabin)
=======
>>>>>>> 5407e32 (prime factors - tabular print)
    fn test_modular_pow() {
        let result = modular_pow(
            &BigInt::from(2u64),
            &BigInt::from(825u64),
            &BigInt::from(173u64),
        );
        assert_eq!(result, BigInt::from(107u64));
    }
}
