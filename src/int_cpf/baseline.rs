use crate::int_cpf::IntCPFValidator;

pub struct IntCPFBaselineValidator;

impl IntCPFValidator for IntCPFBaselineValidator {
    fn new() -> Self {
        IntCPFBaselineValidator {}
    }

    fn validate(&mut self, cpf: u64) -> Result<(), &'static str> {
        if cpf > 999_999_999_99 {
            return Err("CPF value exceeds maximum allowed");
        }

        let mut checksum: u64 = 0;
        {
            let mut factor: u64 = 2;
            let mut div: u64 = 100;

            for _ in 0..9 {
                checksum += ((cpf / div) % 10) * factor;
                div *= 10;
                factor += 1;
            }
        }
        let mut first_remainder: u64 = (checksum * 10) % 11;
        if first_remainder == 10 {
            first_remainder = 0;
        }

        let first_checksum: u64 = (cpf / 10) % 10;
        if first_checksum != first_remainder {
            return Err("Invalid first checksum digit");
        }

        let mut checksum: u64 = first_checksum * 2;
        {
            let mut factor: u64 = 3;
            let mut div: u64 = 100;

            for _ in 0..9 {
                checksum += ((cpf / div) % 10) * factor;
                div *= 10;
                factor += 1;
            }
        }
        let mut second_remainder = (checksum * 10) % 11;
        if second_remainder == 10 {
            second_remainder = 0;
        }

        let second_checksum = cpf % 10;
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
        IntCPFBaselineValidator::new().validate(cpf)
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
        for cpf in generator {
            let int_cpf = cpf_string_to_int(&cpf).expect("Invalid CPF from generator");
            assert!(validate(int_cpf).is_ok(), "Failed for CPF: {}", &cpf);
            if rand::random_bool(0.001f64) {
                println!("Current CPF: {}", CPF::new(cpf));
            }
        }
    }
}
