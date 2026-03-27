use std::fmt::Display;

pub trait CPFValidator {
    fn new() -> Self;
    fn validate(&mut self, cpf: &str) -> Result<(), &'static str>;
}

pub struct CPF(String);

impl CPF {
    pub fn new(cpf: String) -> Self {
        Self(cpf)
    }
}

impl Display for CPF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}-{}", &self.0[0..3], &self.0[3..6], &self.0[6..9], &self.0[9..11])
    }
}
