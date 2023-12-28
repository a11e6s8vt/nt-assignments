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
pub struct NumPQTable {
    number: String,
    factorisation: String,
}

impl NumPQTable {
    pub fn new(number: String, factorisation: String) -> Self {
        Self {
            number,
            factorisation,
        }
    }
}
