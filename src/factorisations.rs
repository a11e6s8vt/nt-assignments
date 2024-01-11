use num_bigint::BigInt;
use num_traits::One;

use crate::{
    primality::miller_rabin_primality,
    utils::{modular_pow, Gcd},
};

pub fn pollards_p_1(n: &BigInt, mut a: BigInt) {
    if miller_rabin_primality(n) {
        println!("{} is a prime", &n);
        return;
    }
    // This list need to be generated dynamically. Static it is for now.
    let prime_powers: Vec<BigInt> = vec![
        BigInt::from(2u64),
        BigInt::from(3u64),
        BigInt::from(2u64),
        BigInt::from(5u64),
        BigInt::from(7u64),
        BigInt::from(2u64),
        BigInt::from(3u64),
        BigInt::from(11u64),
        BigInt::from(13u64),
        BigInt::from(2u64),
        BigInt::from(17u64),
        BigInt::from(19u64),
        BigInt::from(23u64),
        BigInt::from(5u64),
        BigInt::from(3u64),
        BigInt::from(29u64),
        BigInt::from(31u64),
        BigInt::from(2u64),
        BigInt::from(37u64),
        BigInt::from(41u64),
        BigInt::from(43u64),
        BigInt::from(47u64),
        BigInt::from(49u64),
        BigInt::from(53u64),
        BigInt::from(59u64),
        BigInt::from(61u64),
        BigInt::from(2u64),
        BigInt::from(67u64),
        BigInt::from(71u64),
        BigInt::from(73u64),
        BigInt::from(79u64),
        BigInt::from(3u64),
        BigInt::from(83u64),
        BigInt::from(89u64),
        BigInt::from(97u64),
    ];

    for num in prime_powers.iter() {
        let b = modular_pow(&a, num, n);
        let gcd = n.gcd_euclid(&(&b - BigInt::one()));
        println!("{:>10} {:>10} {:>10} {:>5}", num, &a, &b, gcd);
        if &gcd > &BigInt::one() {
            break;
        }
        a = b;
    }
}
