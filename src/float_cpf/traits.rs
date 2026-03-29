use crate::convert::cpf_int_to_string;
use crate::cpf::CPF;
use std::fmt::Display;

pub trait FloatCPFValidator {
    fn new() -> Self;
    fn validate(&mut self, cpf: f64) -> Result<(), &'static str>;
}

pub struct FloatCPF(f64);

impl FloatCPF {
    pub fn new(cpf: f64) -> Self {
        Self(cpf)
    }
}

impl Display for FloatCPF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CPF::new(cpf_int_to_string(self.0 as u64).expect("Invalid CPF")))
    }
}
