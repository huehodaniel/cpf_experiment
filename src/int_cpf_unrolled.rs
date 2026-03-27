use crate::int_cpf::IntCPFValidator;

pub struct UnrolledIntCPFValidator;

impl IntCPFValidator for UnrolledIntCPFValidator {
    fn new() -> Self {
        UnrolledIntCPFValidator {}
    }

    fn validate(&mut self, cpf: u64) -> Result<(), &'static str> {
        if cpf > 999_999_999_99 {
            return Err("CPF value exceeds maximum allowed");
        }

        let d0 = (cpf / 10_000_000_000) % 10;
        let d1 = (cpf / 1_000_000_000) % 10;
        let d2 = (cpf / 100_000_000) % 10;
        let d3 = (cpf / 10_000_000) % 10;
        let d4 = (cpf / 1_000_000) % 10;
        let d5 = (cpf / 100_000) % 10;
        let d6 = (cpf / 10_000) % 10;
        let d7 = (cpf / 1_000) % 10;
        let d8 = (cpf / 100) % 10;

        let first_checksum = (cpf / 10) % 10;
        let second_checksum = cpf % 10;

        let checksum = d0 * 10 + d1 * 9 + d2 * 8 + d3 * 7 + d4 * 6 + d5 * 5 + d6 * 4 + d7 * 3 + d8 * 2;
        let mut first_remainder: u64 = (checksum * 10) % 11;
        if first_remainder == 10 {
            first_remainder = 0;
        }

        if first_checksum != first_remainder {
            return Err("Invalid first checksum digit");
        }

        let checksum = d0 * 11 + d1 * 10 + d2 * 9 + d3 * 8 + d4 * 7 + d5 * 6 + d6 * 5 + d7 * 4 + d8 * 3 + first_checksum * 2;
        let mut second_remainder = (checksum * 10) % 11;
        if second_remainder == 10 {
            second_remainder = 0;
        }
        if second_checksum != second_remainder {
            return Err("Invalid second checksum digit");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::cpf_string_to_int;
    use crate::cpf::CPF;

    fn validate(cpf: u64) -> Result<(), &'static str> {
        UnrolledIntCPFValidator::new().validate(cpf)
    }

    #[test]
    fn test_valid_cpf_naive() {
        let valid_cpf = 529_982_247_25;
        assert!(validate(valid_cpf).is_ok());
    }

    #[test]
    fn test_invalid_length_naive() {
        assert_eq!(
            validate(123456789012),
            Err("CPF value exceeds maximum allowed")
        );
    }

    #[test]
    fn test_invalid_first_checksum() {
        // Valid is 52998224725, changing 9th index (10th digit)
        let invalid_cpf = 529_982_247_35;
        assert_eq!(validate(invalid_cpf), Err("Invalid first checksum digit"));
    }

    #[test]
    fn test_invalid_second_checksum() {
        // Valid is 52998224725, changing 10th index (11th digit)
        let invalid_cpf = 529_982_247_26;
        assert_eq!(validate(invalid_cpf), Err("Invalid second checksum digit"));
    }

    #[test]
    #[ignore]
    fn test_all_cpfs() {
        use crate::generator::CPFGenerator;
        let generator = CPFGenerator::new();
        for cpf in generator.take(100000) {
            let int_cpf = cpf_string_to_int(&cpf).expect("Invalid CPF from generator");
            assert!(validate(int_cpf).is_ok(), "Failed for CPF: {}", &cpf);
            if rand::random_bool(0.001f64) {
                println!("Current CPF: {}", CPF::new(cpf));
            }
        }
    }
}
