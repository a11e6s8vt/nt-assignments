use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use ndarray::Array;
use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_iter::range_inclusive;
use num_traits::One;
use rayon::vec;

use crate::{primality::next_prime, prime_factors::PrimeFactors, utils::modular_pow};

fn zeros(size: usize) -> Vec<i32> {
    let mut zero_vec: Vec<i32> = Vec::with_capacity(size);
    for i in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}

pub fn prepare_matrix(n: &BigInt) {
    let mut primes = vec![BigInt::from(2u64)];
    let a = n.sqrt();
    println!("{}", a);

    let mut factor_base = vec![
        BigInt::from(2u64),
        BigInt::from(5u64),
        BigInt::from(7u64),
        BigInt::from(11u64),
        BigInt::from(13u64),
        BigInt::from(17u64),
        BigInt::from(19u64),
        BigInt::from(23u64),
        BigInt::from(29u64),
        BigInt::from(31u64),
    ];

    for i in factor_base.iter() {
        println!("{}", modular_pow(n, &((i - 1) / BigInt::from(2)), i));
    }
    factor_base.retain(|x| modular_pow(n, &((x - 1) / BigInt::from(2u64)), x) == BigInt::one());
    //factor_base.insert(0, BigInt::from(-1i32));
    let mut LS: HashMap<BigInt, Vec<(BigInt, usize)>> = HashMap::new();
    let start = a.clone() - BigInt::from(100u64);
    let end = a.clone() + BigInt::from(200u64);

    let mut m_by_n: Vec<Vec<i32>> = Vec::new();
    for i in range_inclusive(start, end) {
        let x = &i - &a;
        let mut y = &i * &i - n;
        if y.sign() == Sign::Minus {
            y = -1 * y;
        }
        let p_factors = y.prime_factors(&mut primes).clone();
        let p_factors_map: HashMap<BigInt, i32> = p_factors
            .iter()
            .cloned()
            .map(|(p, e)| (p, e as i32))
            .collect();
        let distinct_factors = p_factors
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<BigInt>>();
        let set1: HashSet<BigInt> = factor_base.iter().cloned().collect();
        let set2: HashSet<BigInt> = distinct_factors.iter().cloned().collect();

        if set2.is_subset(&set1) {
            // println!("{} {} {:?}", i - &a, &y, p_factors);

            let mut one_by_n: Vec<i32> = Vec::new();
            for base in factor_base.iter() {
                if set2.contains(&base) {
                    let e = p_factors_map.get(&base).unwrap();
                    one_by_n.push(e.clone());
                } else {
                    one_by_n.push(0);
                }
            }

            if x.sign() == Sign::Minus {
                one_by_n.insert(0, 1);
            } else {
                one_by_n.insert(0, 0);
            }
            m_by_n.push(one_by_n.clone());
            println!("{}  {:?}", i, one_by_n);
        }
    }
    let mut sum_vec: Vec<i32> = zeros(m_by_n[0].len());
    for row in m_by_n.iter() {
        let mut i = 0;
        while i < row.len() {
            sum_vec[i] += row[i];
            i += 1;
        }
    }
    //let A = Array::from_vec(m_by_n.clone());
    println!("     {:?}", sum_vec);
}
