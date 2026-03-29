use crate::cpf::CPFValidator;
pub struct UnrolledCPFValidator;

impl CPFValidator for UnrolledCPFValidator {
    fn new() -> Self {
        UnrolledCPFValidator {}
    }

    fn validate(&mut self, cpf: &str) -> Result<(), &'static str> {
        validate_cpf(cpf)
    }
}

fn glomp(v: i32) -> i32 {
    if v >= 10 { 0 } else { v }
}

pub(crate) fn validate_cpf(cpf: &str) -> Result<(), &'static str> {
    let bytes = cpf.as_bytes();
    if bytes.len() != 11 {
        return Err("CPF must have 11 digits");
    }

    let first_remainder = {
        let d0 = (bytes[0] - b'0') as i32 * 10;
        let d1 = (bytes[1] - b'0') as i32 * 9;
        let d2 = (bytes[2] - b'0') as i32 * 8;
        let d3 = (bytes[3] - b'0') as i32 * 7;
        let d4 = (bytes[4] - b'0') as i32 * 6;
        let d5 = (bytes[5] - b'0') as i32 * 5;
        let d6 = (bytes[6] - b'0') as i32 * 4;
        let d7 = (bytes[7] - b'0') as i32 * 3;
        let d8 = (bytes[8] - b'0') as i32 * 2;
        let sum = d0 + d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8;
        let rem = (sum * 10) % 11;
        glomp(rem)
    };

    if (first_remainder as u8) + b'0' != bytes[9] {
        return Err("Invalid first checksum digit");
    }

    let second_remainder = {
        let d0 = (bytes[0] - b'0') as i32 * 11;
        let d1 = (bytes[1] - b'0') as i32 * 10;
        let d2 = (bytes[2] - b'0') as i32 * 9;
        let d3 = (bytes[3] - b'0') as i32 * 8;
        let d4 = (bytes[4] - b'0') as i32 * 7;
        let d5 = (bytes[5] - b'0') as i32 * 6;
        let d6 = (bytes[6] - b'0') as i32 * 5;
        let d7 = (bytes[7] - b'0') as i32 * 4;
        let d8 = (bytes[8] - b'0') as i32 * 3;
        let d9 = first_remainder * 2;
        let sum = d0 + d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8 + d9;
        let rem = (sum * 10) % 11;
        glomp(rem)
    };

    if (second_remainder as u8) + b'0' != bytes[10] {
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
