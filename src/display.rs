use clap::builder::Str;
use fmtastic::{Subscript, Superscript};
use num_bigint::BigInt;
use tabled::Tabled;

#[derive(Tabled)]
pub struct GcdTestTable {
    #[tabled(rename = "n = p.q")]
    num: String,
    #[tabled(rename = "a (randomly selected)")]
    a: String,
    #[tabled(rename = "gcd(n, a)")]
    gcd: String,
}

impl GcdTestTable {
    pub fn new(num: String, a: String, gcd: String) -> Self {
        Self { num, a, gcd }
    }
}

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
pub struct NumFactorTable {
    number: String,
    factorisation: String,
}

impl NumFactorTable {
    pub fn new(number: String, factorisation: String) -> Self {
        Self {
            number,
            factorisation,
        }
    }
}
// make the below function generic
pub fn format_prime_factors_print(
    num: &BigInt,
    p_factors: &Vec<(BigInt, usize)>,
    form: &mut String,
    table_data: &mut Vec<NumFactorTable>,
) {
    for (factor, exp) in p_factors {
        form.push_str(&format!("{}{} x ", factor, Superscript(exp.clone())));
    }
    let mut form = form.trim_end().to_string();
    form.pop();
    table_data.push(NumFactorTable::new(num.to_string(), form))
}

#[derive(Tabled)]
pub struct MillerRabinTable<'w> {
    n: BigInt,
    #[tabled(rename = "n - 1 = m.2ˢ")]
    n_minus_one_form: &'w String,
    #[tabled(rename = "a")]
    a: &'w BigInt,
    k: u32,
    #[tabled(rename = "e = m.2ᵏ")]
    e: BigInt,
    #[tabled(rename = "x = aᵉ")]
    a_raised_e: BigInt,
    #[tabled(rename = "x ≡ ±1 (mod n)")]
    x_congruent_1_mod_n: bool,
    message: String,
}

impl<'w> MillerRabinTable<'w> {
    pub fn new(
        n: BigInt,
        n_minus_one_form: &'w String,
        a: &'w BigInt,
        k: u32,
        e: BigInt,
        a_raised_e: BigInt,
        x_congruent_1_mod_n: bool,
        message: String,
    ) -> Self {
        Self {
            n,
            n_minus_one_form,
            a,
            k,
            e,
            a_raised_e,
            x_congruent_1_mod_n,
            message,
        }
    }
}
