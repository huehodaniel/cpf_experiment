use super::FloatCPFValidator;

pub struct FloatCPFBaselineValidator;

impl FloatCPFValidator for FloatCPFBaselineValidator {
    fn new() -> Self {
        FloatCPFBaselineValidator {}
    }

    fn validate(&mut self, cpf: f64) -> Result<(), &'static str> {
        if cpf > 999_999_999_99f64 {
            return Err("CPF value exceeds maximum allowed");
        }

        let mut checksum = 0.0;
        {
            let mut factor = 2.0;
            let mut div = 100.0;

            for _ in 0..9 {
                checksum += ((cpf / div).trunc() % 10.0) * factor;
                div *= 10.0;
                factor += 1.0;
            }
        }
        let first_remainder = ((checksum * 10.0) % 11.0) % 10.0;
        let first_checksum = (cpf / 10.0).trunc() % 10.0;
        if first_checksum != first_remainder {
            return Err("Invalid first checksum digit");
        }

        let mut checksum = first_checksum * 2.0;
        {
            let mut factor = 3.0;
            let mut div = 100.0;

            for _ in 0..9 {
                checksum += ((cpf / div).trunc() % 10.0) * factor;
                div *= 10.0;
                factor += 1.0;
            }
        }
        let second_remainder = ((checksum * 10.0) % 11.0) % 10.0;
        let second_checksum = cpf.trunc() % 10.0;
        if second_checksum != second_remainder {
            return Err("Invalid second checksum digit");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::cpf_string_to_float;
    use crate::cpf::CPF;

    fn validate(cpf: f64) -> Result<(), &'static str> {
        FloatCPFBaselineValidator::new().validate(cpf)
    }

    #[test]
    fn test_valid_cpf_naive() {
        let valid_cpf = 529_982_247_25.0;
        assert!(validate(valid_cpf).is_ok());
    }

    #[test]
    fn test_invalid_length_naive() {
        assert_eq!(
            validate(123456789012.0),
            Err("CPF value exceeds maximum allowed")
        );
    }

    #[test]
    fn test_invalid_first_checksum() {
        // Valid is 52998224725, changing 9th index (10th digit)
        let invalid_cpf = 529_982_247_35.0;
        assert_eq!(validate(invalid_cpf), Err("Invalid first checksum digit"));
    }

    #[test]
    fn test_invalid_second_checksum() {
        // Valid is 52998224725, changing 10th index (11th digit)
        let invalid_cpf = 529_982_247_26.0;
        assert_eq!(validate(invalid_cpf), Err("Invalid second checksum digit"));
    }

    #[test]
    #[ignore]
    fn test_all_cpfs() {
        use crate::generator::CPFGenerator;
        let generator = CPFGenerator::new();
        for cpf in generator {
            let int_cpf = cpf_string_to_float(&cpf).expect("Invalid CPF from generator");
            assert!(validate(int_cpf).is_ok(), "Failed for CPF: {}", &cpf);
            if rand::random_bool(0.001f64) {
                println!("Current CPF: {}", CPF::new(cpf));
            }
        }
    }
}
