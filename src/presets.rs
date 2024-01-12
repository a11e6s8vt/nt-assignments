use crate::{
    display::{
        format_prime_factors_print, matrix_print, miller_rabin_output_print, GcdTestTable, Matrix,
        MillerRabinJson, MillerRabinTable, NumFactorTable,
    },
    groups_modulo_n,
    primality::{
        gcd_test, is_prime_trial_division, is_prime_trial_division_parallel, miller_rabin_test,
    },
    prime_factors::PrimeFactors,
};
use fmtastic::Superscript;
use num_bigint::BigInt;
use num_iter::{range, range_inclusive};

use homedir::get_my_home;
use rand::seq::SliceRandom;
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelBridge,
        ParallelIterator,
    },
    result,
};

use std::io::{ErrorKind, Write};
use std::{
    collections::HashMap,
    fs::{self, File},
};
use std::{
    io::{BufRead, BufReader},
    process::Output,
};

use tabled::{
    col,
    settings::{
        split::Split,
        style::{BorderSpanCorrection, HorizontalLine, On, Style},
        Merge,
    },
    Table,
};

pub enum NumCategory {
    Primes,
    Composites,
    CompositesPQ,
    All,
}

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
    // println!("{:?}", &primes);
    // println!("{:?}", &composites);
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

///
/// Returns a tuple with a formatted string for output and a Vector which contains a tuple of
/// Number and its prime factors
///
/// # Arguments
/// * `start` - BigInt
/// * `end` - BigInt
/// * `NumCategory` - Whether we want the prime factorisation of All numbers or composites or composits of the form P.Q
/// # Example
/// ```
/// use crate::presets::list_prime_factors_in_range;
/// list_prime_factors_in_range(&start, &end, NumCategory::All);
/// ```
pub fn list_prime_factors_in_range(
    start: &BigInt,
    end: &BigInt,
    opts: NumCategory,
) -> (Vec<NumFactorTable>, Vec<(BigInt, Vec<(BigInt, usize)>)>) {
    let mut table_data: Vec<NumFactorTable> = Vec::new();
    let mut primes = vec![BigInt::from(2u64)];
    let mut nums_pfactors: Vec<(BigInt, Vec<(BigInt, usize)>)> = Vec::new();
    for num in range_inclusive(start.clone(), end.clone()) {
        let mut form: String = String::new();
        let p_factors = num.prime_factors(&mut primes);
        match opts {
            NumCategory::All => {
                format_prime_factors_print(&num, &p_factors, &mut form, &mut table_data);
                nums_pfactors.push((num.clone(), p_factors.clone()));
            }
            NumCategory::Composites => {
                if p_factors.len() >= 2 {
                    format_prime_factors_print(&num, &p_factors, &mut form, &mut table_data);
                    nums_pfactors.push((num.clone(), p_factors.clone()));
                }
            }
            NumCategory::CompositesPQ => {
                if p_factors.len() == 2 {
                    let first = p_factors.first().unwrap();
                    let second = p_factors.get(1).unwrap();

                    match first.1 {
                        1 => match second.1 {
                            1 => {
                                format_prime_factors_print(
                                    &num,
                                    &p_factors,
                                    &mut form,
                                    &mut table_data,
                                );
                                nums_pfactors.push((num.clone(), p_factors.clone()));
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            NumCategory::Primes => {}
        }
    }

    (table_data, nums_pfactors)
}

/// Returns a list of Carmichael Numbers (Absolute Pseudoprimes) in a range using FLT or Korselt's criterion
///
/// # Arguments
/// * start: BigInt
/// * end: BigInt
/// * f: a function pointer to either primality::carmichael_nums_korselt or primality::carmichael_nums_flt
/// # Examples
/// ```
/// use crate::presets::list_carmichael_nums;
/// let carmichael_nums = list_carmichael_nums(&start, &end, carmichael_nums_flt);
/// ```
///
pub fn list_carmichael_nums(
    start: &BigInt,
    end: &BigInt,
    f: fn(&BigInt) -> bool,
) -> (String, Vec<(BigInt, Vec<(BigInt, usize)>)>) {
    // Get all the composite numbers in the range
    let composites = list_prime_factors_in_range(start, end, NumCategory::Composites).1;

    // Searching for Carmichael numbers in parallel
    let carmichael_nums = composites
        .par_iter()
        .filter(|x| f(&x.0) == true)
        .map(|x| x.clone())
        .collect::<Vec<(BigInt, Vec<(BigInt, usize)>)>>();

    // Format the data for printing
    let mut table_data: Vec<NumFactorTable> = Vec::new();
    for item in carmichael_nums.iter() {
        let mut form: String = String::new();
        format_prime_factors_print(&item.0, &item.1, &mut form, &mut table_data);
    }

    let mut table1 = Table::new(table_data);
    table1.with(STYLE_2);

    let output1 = table1.to_string();
    (output1, carmichael_nums)
}

pub fn gcd_test_range(start: &BigInt, end: &BigInt) {
    let pq_nums = list_prime_factors_in_range(start, end, NumCategory::Composites);
    let pq_nums = pq_nums.1;

    // This will randomly choose three numbers which are composites in the range given
    let selected_nums_pq = pq_nums
        .choose_multiple(&mut rand::thread_rng(), 3)
        .map(|x| x.clone())
        .collect::<Vec<(BigInt, Vec<(BigInt, usize)>)>>();

    let mut result = Vec::<(BigInt, Vec<(BigInt, usize)>, Vec<(BigInt, BigInt)>)>::new();
    selected_nums_pq
        .par_iter()
        .map(|n| (n.0.clone(), n.1.clone(), gcd_test(&n.0, 4)))
        .collect_into_vec(&mut result);

    let mut table_data = Vec::<GcdTestTable>::new();
    for (num, p_factors, gcd_result) in result {
        let mut form: String = String::new();
        for (factor, exp) in p_factors {
            form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
        }
        let mut form = form.trim_end().to_string();
        form.pop();
        let title = format!("{} = {}", num.to_string(), form);
        for (i, trials) in gcd_result.iter().enumerate() {
            let a = format!("a{} = {}", i + 1, trials.0);
            let gcd = format!("gcd{} = {}", i + 1, trials.1);
            table_data.push(GcdTestTable::new(title.to_owned(), a, gcd));
        }
    }
    let table = Table::new(table_data)
        .with(Merge::vertical())
        .with(Style::modern())
        .with(BorderSpanCorrection)
        .to_string();

    println!("\n{table}\n");
}

pub fn ass1_question3_miller_rabin(n: &BigInt) -> (String, Vec<String>) {
    let mut non_witnesses: Vec<String> = Vec::new();
    let mut n_minus_one_form = String::new();
    for base in range(BigInt::from(2u64), n - 1) {
        let output = miller_rabin_test(&n, Some(&base));
        for item in output.1.iter() {
            if item.get_message().contains("Prime") {
                non_witnesses.push(base.to_string());
                if n_minus_one_form.len() == 0 {
                    n_minus_one_form.push_str(&item.get_n_minus_one_form());
                }
            }
        }
    }
    (n_minus_one_form, non_witnesses)
}

pub fn search_nums_with_primitive_roots(start: BigInt, end: BigInt) -> (Vec<String>, Vec<String>) {
    let mut primes = vec![BigInt::from(2u64)];

    let mut nums_with_prim_roots: Vec<String> = Vec::new();
    let mut nums_without_no_prim_roots: Vec<String> = Vec::new();

    for i in range_inclusive(start, end) {
        let prim_roots_i = groups_modulo_n::primitive_roots_trial_n_error(&i);
        if prim_roots_i.len() > 0 {
            nums_with_prim_roots.push(i.to_string());
        } else {
            nums_without_no_prim_roots.push(i.to_string());
        }
    }
    (nums_with_prim_roots, nums_without_no_prim_roots)
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
