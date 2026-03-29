use crate::cpf::CPFValidator;
pub struct BaselineCPFValidator;

impl CPFValidator for BaselineCPFValidator {
    fn new() -> Self {
        BaselineCPFValidator {}
    }

    fn validate(&mut self, cpf: &str) -> Result<(), &'static str> {
        validate_cpf(cpf)
    }
}

pub(crate) fn validate_cpf(cpf: &str) -> Result<(), &'static str> {
    let bytes = cpf.as_bytes();
    if bytes.len() != 11 {
        return Err("CPF must have 11 digits");
    }

    let mut first_checksum: i32 = 0;
    {
        let mut factor: i32 = 10;
        for x in bytes[0..9].iter() {
            first_checksum += (*x - b'0') as i32 * factor;
            factor -= 1;
        }
    }

    let first_remainder = ((first_checksum * 10) % 11) % 10;
    if (first_remainder as u8) + b'0' != bytes[9] {
        return Err("Invalid first checksum digit");
    }

    let mut second_checksum: i32 = 0;
    {
        let mut factor: i32 = 11;
        for x in bytes[0..10].iter() {
            second_checksum += (*x - b'0') as i32 * factor;
            factor -= 1;
        }
    }

    let second_checksum = ((second_checksum * 10) % 11) % 10;
    if (second_checksum as u8) + b'0' != bytes[10] {
        return Err("Invalid second checksum digit");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpf::CPF;

    #[test]
    fn test_valid_cpf() {
        let valid_cpf = "52998224725";
        assert!(validate_cpf(valid_cpf).is_ok());
    }

    #[test]
    fn test_invalid_length() {
        assert_eq!(validate_cpf("123"), Err("CPF must have 11 digits"));
        assert_eq!(validate_cpf("123456789012"), Err("CPF must have 11 digits"));
    }

    #[test]
    fn test_invalid_first_checksum() {
        // Valid is 52998224725, changing 9th index (10th digit)
        let invalid_cpf = "52998224735";
        assert_eq!(
            validate_cpf(invalid_cpf),
            Err("Invalid first checksum digit")
        );
    }

    #[test]
    fn test_invalid_second_checksum() {
        // Valid is 52998224725, changing 10th index (11th digit)
        let invalid_cpf = "52998224726";
        assert_eq!(
            validate_cpf(invalid_cpf),
            Err("Invalid second checksum digit")
        );
    }

    #[test]
    #[ignore]
    fn test_all_cpfs() {
        use crate::generator::CPFGenerator;
        let generator = CPFGenerator::new();
        for cpf in generator {
            assert!(validate_cpf(&cpf).is_ok(), "Failed for CPF: {}", cpf);
            if rand::random_bool(0.00001f64) {
                println!("Current CPF: {}", CPF::new(cpf));
            }
        }
    }
}
