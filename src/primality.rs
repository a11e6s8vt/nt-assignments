use crate::{
    display::{format_miller_rabin_steps_print, MillerRabinTable},
    prime_factors::PrimeFactors,
    utils::{coprime_nums_less_than_n, generate_random_int_in_range, modular_pow, Gcd},
};
use fmtastic::Superscript;
use num_bigint::BigInt;
use num_iter::range_inclusive;
use num_traits::{One, Zero};
use rayon::prelude::*;

use tabled::settings::style::{HorizontalLine, On, Style};

const STYLE_2: Style<On, On, On, On, On, On, 0, 0> = Style::rounded()
    .line_horizontal(HorizontalLine::inherit(Style::modern()))
    .remove_horizontals();

///
/// is_prime calculates if a number is prime by verifying numbers upto √n.
///
pub fn is_prime_trial_division(n: &BigInt) -> bool {
    let (zero, one, _two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let three = BigInt::from(3u64);
    // returns true if the number is 2 or 3
    if n <= &three {
        return n > &one;
    }

    if n % 2 == zero || n % 3 == zero {
        return false;
    }

    let square_root = n.sqrt() + 1; // +1 to get the ceiling value
    for i in range_inclusive(BigInt::from(5u64), square_root).step_by(6) {
        if n % &i == zero || n % (&i + 2) == zero {
            return false;
        }
    }

    true
}

pub fn is_prime_trial_division_parallel(n: &BigInt) -> bool {
    let (zero, one, _two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let three = BigInt::from(3u64);

    // returns true if the number is 2 or 3
    if n <= &three {
        return n > &one;
    }

    if n % 2 == zero || n % 3 == zero {
        return false;
    }

    let upper_bound = n.sqrt() + 1; // +1 to get the ceiling value

    if let Some(_divisor) = range_inclusive(BigInt::from(5u64), upper_bound)
        .par_bridge()
        .into_par_iter()
        .find_first(|divisor| n % divisor == zero)
    {
        false
    } else {
        true
    }
}

/// https://www.youtube.com/watch?v=SSpcBIM9Gb8
/// Miller-Rabin Test Step-1
/// It accepts an integer and returns a boolean value
/// 1. Express n - 1 as 2ᶠm
pub fn miller_rabin_primality(n: &BigInt) -> bool {
    let (zero, one, _two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let three = BigInt::from(3u64);
    if n <= &one || n == &BigInt::from(4u64) {
        return false;
    }
    if n <= &three {
        return true;
    }

    let mut d: BigInt = n - &one;
    // Express n - 1 as 2ᶠ.m
    while &d % 2 == zero {
        d = &d / 2;
    }
    // d = (n - 1) / 2ᶠ

    for _ in 0..5 {
        if miller_test(&d, n) == false {
            // If miller-rabin test returns false once, the given integer
            // is not a prime
            return false;
        }
    }

    true
}

/// Miller-Rabin Test - Step 2
///
fn miller_test(d: &BigInt, n: &BigInt) -> bool {
    let (_zero, one, two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let mut d = d.clone();
    // Randomly generate a base: a such that 1 < a < n - 1
    let a: BigInt = generate_random_int_in_range(&two, &(n - 1));

    // Calculate x ≡ a^d(mod n)
    let mut x = modular_pow(&a, &d, n);

    // if x ≡ ±1 (mod n), return true
    if x == one || x == n - 1 {
        return true;
    }

    // if x ≢ ±1 (mod n), while d != n-1 .
    // d was obtained by repeated division of (m - 1) by 2.
    // multiplying it with 2 repeatedly until it equals (m - 1)
    while d != n - 1 {
        // sqaure x - This is a^((2^j)m)(mod n)
        x = modular_pow(&x, &two, n);

        // if x ≡ -1 (mod n) the input number is probably prime
        if x == n - 1 {
            return true;
        }

        // if x ≡ -1 (mod n), then x is a factor of n
        if x == one {
            return false;
        }

        // multiplication by 2
        d = d * &two;
    }

    false
}

///
/// Miller-Rabin Test
///
pub fn miller_rabin_test(n: &BigInt, base: &BigInt) -> (bool, Vec<MillerRabinTable>) {
    let mut table_data: Vec<MillerRabinTable> = Vec::new();
    let _message = String::new();
    let _is_prime = false;
    let (zero, one, two) = (BigInt::from(0u64), BigInt::from(1u64), BigInt::from(2u64));
    let n_minus_one: BigInt = n - 1;
    let mut m = n_minus_one.clone();

    let mut s = 0;
    while &m % 2 == zero {
        m /= 2;
        s += 1;
    }

    let n_minus_one_form = format!("{} = {}.2{}", n_minus_one, m, Superscript(s),);

    // Randomly generate a base "a" such that 1 < a < n - 1
    let a: BigInt = generate_random_int_in_range(&two, &(n - 1));
    // let a = BigInt::from(1003u64);

    // Calculate x ≡ aᵐ(mod n)
    let mut x = modular_pow(base, &m, n);

    format_miller_rabin_steps_print(
        n.clone(),
        &n_minus_one_form,
        s,
        base.clone(),
        0,
        m.clone(),
        x.clone(),
        &x == &one,
        &x == &(n - 1),
        &mut table_data,
    );

    // if x ≡ ±1 (mod n),
    // Why? We know that aⁿ⁻¹ ≡ (aᵐ²^ˢ) ≡ 1 (mod n), and we will not
    // find a square root of 1, other than ±1, in repeated squaring of am
    // to get an−1.
    if &x == &one || &x == &(n - 1) {
        return (true, table_data);
    }

    let mut k = 1;
    while k <= s {
        // loop for searching square-roots for 1 (mod n) other than ±1 (mod n)
        let e = &m * BigInt::from(2u64).pow(k);
        x = modular_pow(&a, &e, n);

        format_miller_rabin_steps_print(
            n.clone(),
            &n_minus_one_form,
            s,
            base.clone(),
            k,
            e.clone(),
            x.clone(),
            &x == &one,
            &x == &(n - 1),
            &mut table_data,
        );

        // if x ≡ -1 (mod n) the input number is probably prime
        if x == n - 1 {
            return (true, table_data);
        }

        // if x ≡ 1 (mod n), then x is a factor of n
        if &x == &one {
            return (false, table_data);
        }

        k += 1;
    }

    // a^n-1(mod n)≢ 1, then by FLT, n is composite and return false.
    return (false, table_data);
}

pub fn gcd_test(n: &BigInt, num_trials: u8) -> Vec<(BigInt, BigInt)> {
    let mut r = Vec::<BigInt>::new();
    for _ in 0..num_trials {
        r.push(generate_random_int_in_range(&BigInt::from(2u8), &(n - 1)));
    }

    let mut result = Vec::<(BigInt, BigInt)>::new();
    for a in r.iter() {
        result.push((a.clone(), n.gcd_euclid(&a)));
    }

    result
}

///
/// Carmichael Numbers using FLT
/// n: a composite number
///
pub fn carmichael_nums_flt(n: &BigInt) -> bool {
    let n_minus_one = n - 1;
    let coprimes_n = coprime_nums_less_than_n(n);
    let fermat_witnesses = coprimes_n
        .par_iter()
        .filter(|x| modular_pow(&x, &n_minus_one, n) != BigInt::one())
        .map(|x| x.clone())
        .collect::<Vec<BigInt>>();

    // No Fermat Witness means n is a Carmichael Number
    fermat_witnesses.len() == 0
}

///
/// Carmichael Numbers using Korselt's criteria
/// n: a composite number
///
pub fn carmichael_nums_korselt(n: &BigInt) -> bool {
    let mut primes = vec![BigInt::from(2u64)];
    let p_factors = n.prime_factors(&mut primes);
    let squarefree = p_factors.iter().fold(true, |squarefree: bool, factor| {
        squarefree & (factor.1 == 1)
    });
    let mut p_m_o_divides_n_m_o = true;
    if squarefree {
        let n_minus_one = n - 1;
        for (p, _) in p_factors.iter() {
            p_m_o_divides_n_m_o &= &n_minus_one % (p - 1) == BigInt::zero();
        }
    }

    squarefree & p_m_o_divides_n_m_o
}

///
/// AKS Primality test
///
pub fn aks(_n: &BigInt) {
    fn is_perfect_k_th_power(n: &BigInt) -> bool {
        let upper_bound = n.sqrt();
        for k in range_inclusive(BigInt::from(2u64), upper_bound) {
            let mut m = n.clone();
            let mut j = BigInt::zero();
            while &m % &k == BigInt::zero() && m > BigInt::one() {
                m /= &k;
                j += 1;
            }
            if m == BigInt::one() && j > BigInt::one() {
                println!("{} is perfect k th power", &n);
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_gcd_euclid_1() {
    //     let result = gcd_euclid(100, 76, false);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn test_get_integers_coprime_n_1() {
    //     let result = get_integers_coprime_n(10);
    //     assert_eq!(result, vec![1, 3, 7, 9]);
    // }

    // #[test]
    // fn test_get_integers_coprime_n_2() {
    //     let result = get_integers_coprime_n(17);
    //     let s = (1..17).collect::<Vec<u64>>();
    //     assert_eq!(result, s);
    // }

    #[test]
    fn test_is_prime_1() {
        let result = is_prime_trial_division(&BigInt::from(409u64));
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_prime_2() {
        let result = is_prime_trial_division(&BigInt::from(1363u64));
        assert_eq!(result, false);
        let result = is_prime_trial_division_parallel(&BigInt::from(37909u64));
        assert_eq!(result, false);
        let result = is_prime_trial_division_parallel(&BigInt::from(37949u64));
        assert_eq!(result, false);
        let result = is_prime_trial_division(&BigInt::from(37979u64));
        assert_eq!(result, false);
    }

    #[test]
    fn test_miller_rabin_primality_1() {
        let result = miller_rabin_primality(&BigInt::from(409u64));
        assert_eq!(result, true);
    }

    #[test]
    fn test_miller_rabin_primality_2() {
        let result = miller_rabin_primality(&BigInt::from(511u64));
        assert_eq!(result, false);
        let result = miller_rabin_primality(&BigInt::from(721u64));
        assert_eq!(result, false);
        let result = miller_rabin_primality(&"49675218696612399034240799519655205503986657506787162015105425670413948962864456158664793804627084299081036134562339483478437262146378569515417671690110863951848724044479367633926630234074394356492223".parse::<BigInt>().unwrap());
        assert_eq!(result, true);
    }

    // #[test]
    // fn test_prime_factors() {
    //     let result = prime_factors(100);
    //     assert_eq!(result, vec![(2, 2), (5, 2)]);
    // }

    // #[test]
    // fn test_divisors_of_n() {
    //     let result = divisors_of_n(160);
    //     let d: Vec<u64> = vec![1, 2, 4, 5, 8, 10, 16, 20, 32, 40, 80, 160];
    //     assert_eq!(result, d);
    // }

    // #[test]
    // fn test_euler_totient_phi_v1() {
    //     let result = euler_totient_phi_v1(378);
    //     assert_eq!(result, 108);

    //     let result = euler_totient_phi_v1(601);
    //     assert_eq!(result, 600);
    // }

    // #[test]
    // fn test_euler_totient() {
    //     let result = euler_totient_phi(378);
    //     assert_eq!(result, 108);
    // }

    // #[test]
    // fn test_primitive_roots_trial_n_error() {
    //     let result = primitive_roots_trial_n_error(25);
    //     assert_eq!(result, vec![2, 3, 8, 12, 13, 17, 22, 23])
    // }

    // #[test]
    // fn test_primitive_roots_count_modulo_n() {
    //     let result = primitive_roots_count_modulo_n(1250);
    //     assert_eq!(result, 200);
    //     let result = primitive_roots_count_modulo_n(59);
    //     assert_eq!(result, 28);
    //     let result = primitive_roots_count_modulo_n(20);
    //     assert_eq!(result, 0);
    //     let result = primitive_roots_count_modulo_n(30);
    //     assert_eq!(result, 0);
    //     let result = primitive_roots_count_modulo_n(10);
    //     assert_eq!(result, 2);
    //     let result = primitive_roots_count_modulo_n(40);
    //     assert_eq!(result, 0);
    // }
}
