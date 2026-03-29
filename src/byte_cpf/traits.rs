pub trait ByteCPFValidator {
    fn new() -> Self;
    fn validate(&mut self, cpf: &[u8]) -> Result<(), &'static str>;
}
