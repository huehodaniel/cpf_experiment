use crate::byte_cpf::ByteCPFValidator;
pub struct BaselineByteCPFValidator;

impl ByteCPFValidator for BaselineByteCPFValidator {
    fn new() -> Self {
        BaselineByteCPFValidator {}
    }

    fn validate(&mut self, cpf: &[u8]) -> Result<(), &'static str> {
        validate_cpf(cpf)
    }
}

pub(crate) fn validate_cpf(cpf: &[u8]) -> Result<(), &'static str> {
    if cpf.len() != 11 {
        return Err("CPF must have 11 digits");
    }

    let mut first_checksum: u32 = 0;
    {
        let mut factor: u32 = 10;
        for x in cpf[0..9].iter() {
            first_checksum += *x as u32 * factor;
            factor -= 1;
        }
    }

    let first_remainder = ((first_checksum * 10) % 11) % 10;
    if (first_remainder as u8) != cpf[9] {
        return Err("Invalid first checksum digit");
    }

    let mut second_checksum: i32 = 0;
    {
        let mut factor: i32 = 11;
        for x in cpf[0..10].iter() {
            second_checksum += *x as i32 * factor;
            factor -= 1;
        }
    }

    let second_checksum = ((second_checksum * 10) % 11) % 10;
    if (second_checksum as u8) != cpf[10] {
        return Err("Invalid second checksum digit");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::cpf_string_to_byte as as_byte_cpf;
    use crate::cpf::CPF;

    #[test]
    fn test_valid_cpf() {
        let valid_cpf = as_byte_cpf("52998224725");
        assert!(validate_cpf(&valid_cpf).is_ok());
    }

    #[test]
    fn test_invalid_length() {
        assert_eq!(validate_cpf(&as_byte_cpf("123")), Err("CPF must have 11 digits"));
        assert_eq!(validate_cpf(&as_byte_cpf("123456789012")), Err("CPF must have 11 digits"));
    }

    #[test]
    fn test_invalid_first_checksum() {
        // Valid is 52998224725, changing 9th index (10th digit)
        let invalid_cpf = as_byte_cpf("52998224735");
        assert_eq!(
            validate_cpf(&invalid_cpf),
            Err("Invalid first checksum digit")
        );
    }

    #[test]
    fn test_invalid_second_checksum() {
        // Valid is 52998224725, changing 10th index (11th digit)
        let invalid_cpf = as_byte_cpf("52998224726");
        assert_eq!(
            validate_cpf(&invalid_cpf),
            Err("Invalid second checksum digit")
        );
    }

    #[test]
    #[ignore]
    fn test_all_cpfs() {
        use crate::generator::CPFGenerator;
        let generator = CPFGenerator::new();
        for cpf in generator {
            assert!(validate_cpf(&as_byte_cpf(&cpf)).is_ok(), "Failed for CPF: {}", cpf);
            if rand::random_bool(0.00001f64) {
                println!("Current CPF: {}", CPF::new(cpf));
            }
        }
    }
}
