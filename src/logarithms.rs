use num_bigint::BigInt;
use num_integer::Integer;
use num_iter::range;
use num_traits::{One, Zero};

use crate::display::PollardsRhoJson;

fn new_xab(
    mut x: BigInt,
    mut ai: BigInt,
    mut bi: BigInt,
    a: &BigInt,
    b: &BigInt,
    n: &BigInt,
) -> (BigInt, BigInt, BigInt) {
    let (_, rem) = x.div_rem(&BigInt::from(3u64));
    if rem == BigInt::zero() {
        x = (&x * &x) % n;
        ai = ai * 2 % (n - 1);
        bi = bi * 2 % (n - 1);
    }

    if rem == BigInt::one() {
        x = (x * a) % n;
        ai = (ai + 1) % (n - 1);
    }

    if rem == BigInt::from(2u64) {
        x = (x * b) % n;
        bi = (bi + 1) % (n - 1);
    }
    (x, ai, bi)
}

pub fn pollards_rho(a: &BigInt, b: &BigInt, n: &BigInt) -> Vec<PollardsRhoJson> {
    let mut result = Vec::<PollardsRhoJson>::new();
    let (mut x1, mut a1, mut b1) = (BigInt::one(), BigInt::zero(), BigInt::zero());
    let (mut x2, mut a2, mut b2) = (x1.clone(), a1.clone(), b1.clone());
    for i in range(BigInt::one(), n.clone()) {
        (x1, a1, b1) = new_xab(x1, a1, b1, &a, &b, n);
        (x2, a2, b2) = new_xab(x2, a2, b2, &a, &b, n);
        (x2, a2, b2) = new_xab(x2, a2, b2, &a, &b, n);
        result.push(PollardsRhoJson::new(
            i.to_string(),
            x1.to_string(),
            a1.to_string(),
            b1.to_string(),
            x2.to_string(),
            a2.to_string(),
            b2.to_string(),
        ));
        if x1 == x2 {
            break;
        }
    }

    result
}
