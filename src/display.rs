use clap::builder::Str;
use fmtastic::Superscript;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use num_traits::ToPrimitive;
use std::iter;
use table_to_html::{Alignment, Entity, HtmlTable};
use tabled::{
    col, row,
    settings::{
        panel::Header,
        split::Split,
        style::{BorderSpanCorrection, HorizontalLine, On, Style},
        Merge, Padding, Panel,
    },
    Table, Tabled,
};

const STYLE_2: Style<On, On, On, On, On, On, 0, 0> = Style::rounded()
    .line_horizontal(HorizontalLine::inherit(Style::modern()))
    .remove_horizontals();

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

#[derive(Tabled, Serialize, Deserialize)]
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

impl std::fmt::Display for NumFactorTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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

#[derive(Clone, Debug, Tabled, Serialize, Deserialize)]
pub struct MillerRabinTable {
    n: String,
    #[tabled(rename = "n - 1 = m.2ˢ")]
    n_minus_one_form: String,
    s: String,
    #[tabled(rename = "a")]
    a: String,
    k: String,
    #[tabled(rename = "e = m.2ᵏ")]
    e: String,
    #[tabled(rename = "x = aᵉ")]
    a_raised_e: String,
    #[tabled(rename = "x ≡ 1 (mod n)")]
    x_congruent_1_mod_n: bool,
    #[tabled(rename = "x ≡ -1 (mod n)")]
    x_congruent_minus_1_mod_n: bool,
    message: String,
}

impl MillerRabinTable {
    pub fn new(
        n: String,
        n_minus_one_form: String,
        s: String,
        a: String,
        k: String,
        e: String,
        a_raised_e: String,
        x_congruent_1_mod_n: bool,
        x_congruent_minus_1_mod_n: bool,
        message: String,
    ) -> Self {
        Self {
            n,
            n_minus_one_form,
            s,
            a,
            k,
            e,
            a_raised_e,
            x_congruent_1_mod_n,
            x_congruent_minus_1_mod_n,
            message,
        }
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

// make the below function generic
pub fn format_miller_rabin_steps_print(
    n: BigInt,
    n_minus_one_form: &String,
    s: u32,
    a: BigInt,
    k: u32,
    e: BigInt,
    a_raised_e: BigInt,
    x_congruent_1_mod_n: bool,
    x_congruent_minus_1_mod_n: bool,
    table_data: &mut Vec<MillerRabinTable>,
) {
    let mut message: String = String::new();
    if &k == &0 {
        if x_congruent_1_mod_n || x_congruent_minus_1_mod_n {
            message.push_str(&format!("{} is Probably Prime", n));
        } else {
            message.push_str(&format!(
                "{} is neither congruent to 1 (mod n) nor -1 (mod n). Search for sqaure roots of 1 (mod n)",
                a_raised_e
            ));
        }
    } else if &k < &s {
        if x_congruent_minus_1_mod_n {
            message.push_str(&format!("{} is Probably Prime", n));
        } else if x_congruent_1_mod_n {
            message.push_str(&format!("{} is composite", n));
        }
    } else if &k == &s {
        message.push_str(&format!("{} is composite", n));
    }

    table_data.push(MillerRabinTable::new(
        n.to_string(),
        n_minus_one_form.clone(),
        s.to_string(),
        a.to_string(),
        k.to_string(),
        e.to_string(),
        a_raised_e.to_string(),
        x_congruent_1_mod_n,
        x_congruent_minus_1_mod_n,
        message,
    ));
}

pub fn miller_rabin_output_print(table_data: &Vec<MillerRabinTable>) {
    let _table = Table::new(table_data)
        .with(Merge::vertical())
        .with(Style::modern())
        .with(BorderSpanCorrection)
        .to_string();

    // let json = serde_json::to_string(table_data).unwrap();
    // println!("{}", json);
    // let mut table1 = Table::new(table_data);
    // table1.with(STYLE_2);

    // let output1 = table1.to_string();
    // println!("\n{}\n", table);
    // let mut html_table =
    //     HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(table_data)));
    // html_table.set_alignment(Entity::Row(1), Alignment::center());
    // html_table.set_border(3);
    // println!("{html_table}");
}

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
pub struct Matrix {
    number: String,
}

impl Matrix {
    pub fn new(number: String) -> Self {
        Self { number }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn matrix_print<T: std::fmt::Display + Tabled>(
    table_data: &Vec<T>,
    title: String,
    split_index: usize,
) {
    let mut table = Table::new(table_data.into_iter());
    table.with(Style::modern());

    //let table_1 = table.clone().with(Split::column(2)).clone();
    let table_5 = table
        .clone()
        .with(Split::row(split_index).concat())
        .to_string();

    let mut table = col![
        row![col![table_5].with(Style::blank()).with(Padding::zero())]
            .with(Panel::header(title))
            .with(Style::blank())
            .with(Padding::zero()),
    ];
    table.with(Style::blank());

    println!("\n{table}\n");
}
