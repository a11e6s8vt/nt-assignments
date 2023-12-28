use num_bigint::BigInt;
use tabled::{
    settings::{
        style::{BorderSpanCorrection, Style},
        Merge,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct GcdTestTable {
    #[tabled(rename = "n = p.q")]
    num: String,
    #[tabled(rename = "a (randomly selected)")]
    a: BigInt,
    #[tabled(rename = "gcd(n, a)")]
    gcd: BigInt,
}

impl GcdTestTable {
    fn new(num: String, a: BigInt, gcd: BigInt) -> Self {
        Self { num, a, gcd }
    }
}
