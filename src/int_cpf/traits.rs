use crate::convert::cpf_int_to_string;
use crate::cpf::CPF;
use std::fmt::Display;

pub trait IntCPFValidator {
    fn new() -> Self;
    fn validate(&mut self, cpf: u64) -> Result<(), &'static str>;
}

pub struct IntCPF(u64);

impl IntCPF {
    pub fn new(cpf: u64) -> Self {
        Self(cpf)
    }
}

impl Display for IntCPF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CPF::new(cpf_int_to_string(self.0).expect("Invalid CPF")))
    }
}