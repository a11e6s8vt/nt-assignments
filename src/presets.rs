use crate::{
    primality::{is_prime_trial_division, is_prime_trial_division_parallel},
    prime_factors::PrimeFactors,
};
use fmtastic::{Subscript, Superscript};
use num_bigint::BigInt;
use num_iter::{range, range_inclusive};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use tabled::{
    grid::config::Borders,
    settings::{
        style::{HorizontalLine, On, Style},
        Border,
    },
    Table, Tabled,
};

const STYLE_2: Style<On, On, On, On, On, On, 0, 0> = Style::rounded()
    .line_horizontal(HorizontalLine::inherit(Style::modern()))
    .remove_horizontals();

pub fn find_primes_in_range_trial_division_parallel(
    start: BigInt,
    end: BigInt,
) -> (Vec<BigInt>, Vec<BigInt>) {
    let nums_categorised = range_inclusive(start, end)
        .par_bridge()
        .into_par_iter()
        .map(|x| (x.clone(), is_prime_trial_division_parallel(&x)))
        .collect::<Vec<(BigInt, bool)>>();

    let mut primes: Vec<BigInt> = Vec::new();
    let mut composites: Vec<BigInt> = Vec::new();
    for x in nums_categorised {
        if x.1 == true {
            primes.push(x.0)
        } else {
            composites.push(x.0)
        }
    }
    primes.sort();
    composites.sort();

    (primes, composites)
}

pub fn find_primes_in_range_trial_division(
    start: BigInt,
    end: BigInt,
) -> (Vec<BigInt>, Vec<BigInt>) {
    let nums_categorised = range_inclusive(start, end)
        .into_iter()
        .map(|x| (x.clone(), is_prime_trial_division(&x)))
        .collect::<Vec<(BigInt, bool)>>();

    let mut primes: Vec<BigInt> = Vec::new();
    let mut composites: Vec<BigInt> = Vec::new();
    for x in nums_categorised {
        if x.1 == true {
            primes.push(x.0)
        } else {
            composites.push(x.0)
        }
    }
    primes.sort();
    composites.sort();

    (primes, composites)
}

pub fn list_prime_factors_in_range(start: &BigInt, end: &BigInt) {
    let mut data: Vec<(String, String)> = Vec::new();
    let mut primes = vec![BigInt::from(2u64)];
    for num in range(start.clone(), end.clone()) {
        let mut form: String = String::new();
        let p_factors = num.prime_factors(&mut primes);
        for (factor, exp) in p_factors {
            form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
        }
        let mut form = form.trim_end().to_string();
        form.pop();
        data.push((num.to_string(), form))
    }

    let mut table1 = Table::new(data);
    table1.with(STYLE_2);

    let output1 = table1.to_string();
    println!("{}", output1);
}

pub fn list_prime_factors_in_range_form_pq(start: &BigInt, end: &BigInt) {
    let mut data: Vec<(String, String)> = Vec::new();
    for num in range(start.clone(), end.clone()) {
        let mut form: String = String::new();
        let (is_pq, p_factors) = num.is_prime_factors_form_pq();
        for (factor, exp) in p_factors {
            form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
        }
        let mut form = form.trim_end().to_string();
        form.pop();

        if is_pq {
            data.push((num.to_string(), form))
        }
    }

    let mut table1 = Table::new(data);
    table1.with(STYLE_2);

    let output1 = table1.to_string();
    println!("{}", output1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> (BigInt, BigInt, Vec<BigInt>) {
        let start = BigInt::from(2800u64);
        let end = BigInt::from(3100u64);

        let primes_in_range = vec![
            BigInt::from(2801u64),
            BigInt::from(2803u64),
            BigInt::from(2819u64),
            BigInt::from(2833u64),
            BigInt::from(2837u64),
            BigInt::from(2843u64),
            BigInt::from(2851u64),
            BigInt::from(2857u64),
            BigInt::from(2861u64),
            BigInt::from(2879u64),
            BigInt::from(2887u64),
            BigInt::from(2897u64),
            BigInt::from(2903u64),
            BigInt::from(2909u64),
            BigInt::from(2917u64),
            BigInt::from(2927u64),
            BigInt::from(2939u64),
            BigInt::from(2953u64),
            BigInt::from(2957u64),
            BigInt::from(2963u64),
            BigInt::from(2969u64),
            BigInt::from(2971u64),
            BigInt::from(2999u64),
            BigInt::from(3001u64),
            BigInt::from(3011u64),
            BigInt::from(3019u64),
            BigInt::from(3023u64),
            BigInt::from(3037u64),
            BigInt::from(3041u64),
            BigInt::from(3049u64),
            BigInt::from(3061u64),
            BigInt::from(3067u64),
            BigInt::from(3079u64),
            BigInt::from(3083u64),
            BigInt::from(3089u64),
        ];
        (start, end, primes_in_range)
    }

    #[test]
    fn test_find_primes_in_range_trial_division_parallel() {
        let (start, end, primes_in_range) = test_data();
        assert_eq!(
            primes_in_range,
            find_primes_in_range_trial_division_parallel(start, end).0
        );
    }

    #[test]
    fn test_find_primes_in_range_trial_division() {
        let (start, end, primes_in_range) = test_data();
        assert_eq!(
            primes_in_range,
            find_primes_in_range_trial_division(start, end).0
        );
    }
}
