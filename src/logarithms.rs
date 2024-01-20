use num_bigint::BigInt;
use num_integer::Integer;
use num_iter::range;
use num_traits::{One, Zero};
use tabled::{settings::Style, Table};

use crate::{display::PollardsRhoJson, utils::Gcd};

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

pub fn pollards_rho(a: &BigInt, b: &BigInt, n: &BigInt) {
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
            let mut table = Table::new(&result);

            table.with(Style::modern());
            println!("\n{}\n", table.to_string());
            let mut b2_1 = &b2 - &b1;
            let mut a1_2 = &a1 - &a2;
            let d = b2_1.gcd_euclid(&(n - &BigInt::one()));

            if &d == &BigInt::one() {
                println!(
                    "Solve the congruence equation: {}x ≡ {} (mod {})",
                    &b2_1,
                    &a1_2,
                    n - 1
                );
                break;
            }
            if &a1_2 % &d != BigInt::zero() {
                println!("No solutions!");
                break;
            }
            b2_1 = &b2_1 / &d;
            a1_2 = &a1_2 / &d;

            println!(
                "Solve the congruence equation: {}x ≡ {} (mod {})",
                &b2_1,
                &a1_2,
                (n - 1) / d
            );
            break;
        }
    }
}
