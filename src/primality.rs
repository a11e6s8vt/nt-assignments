use num_bigint::BigInt;
use num_iter::range_inclusive;
use rayon::prelude::*;

///
/// is_prime calculates if a number is prime by verifying numbers upto √n.
///
pub fn is_prime_trial_division(n: &BigInt) -> bool {
    // returns true if the number is 2 or 3
    let zero = BigInt::from(0u64);
    let one = BigInt::from(1u64);
    let three = BigInt::from(3u64);
    if n <= &three {
        return n > &one;
    }

    if n % 2 == zero || n % 3 == zero {
        return false;
    }

    let square_root = n.sqrt() + 1; // +1 to get the ceiling value
    for i in range_inclusive(BigInt::from(5u64), square_root).step_by(6) {
        if n % &i == BigInt::from(0u64) || n % (&i + 2) == BigInt::from(0u64) {
            return false;
        }
    }

    true
}

pub fn is_prime_trial_division_parallel(n: &BigInt) -> bool {
    // returns true if the number is 2 or 3
    let zero = BigInt::from(0u64);
    let one = BigInt::from(1u64);
    let three = BigInt::from(3u64);
    if n <= &three {
        return n > &one;
    }

    if n % 2 == zero || n % 3 == zero {
        return false;
    }

    let upper_bound = n.sqrt() + 1; // +1 to get the ceiling value

    if let Some(divisor) = range_inclusive(BigInt::from(5u64), upper_bound)
        .par_bridge()
        .into_par_iter()
        .find_first(|divisor| n % divisor == BigInt::from(0u64))
    {
        false
    } else {
        true
    }
}
