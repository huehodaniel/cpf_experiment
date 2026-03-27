use crate::cpf::CPFValidator;
use std::simd::prelude::*;

pub struct SimdCPFValidator;

impl CPFValidator for SimdCPFValidator {
    fn new() -> Self {
        SimdCPFValidator {}
    }

    fn validate(&mut self, cpf: &str) -> Result<(), &'static str> {
        validate_cpf(cpf)
    }
}

const CHECKSUM_MASK_1ST: Simd<u16, 16> = u16x16::from_array([10, 9, 8, 7, 6, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
const CHECKSUM_MASK_2ND: Simd<u16, 16> = u16x16::from_array([11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0]);
const ZERO_MASK: Simd<u8, 16> = Simd::splat(b'0');

pub(crate) fn validate_cpf(cpf: &str) -> Result<(), &'static str> {
    let bytes = cpf.as_bytes();
    if bytes.len() != 11 {
        return Err("CPF must have 11 digits");
    }

    let block: Simd<u16, 16> = (u8x16::load_or_default(bytes) - ZERO_MASK).cast();

    let first_product = (block * CHECKSUM_MASK_1ST).reduce_sum();
    let first_remainder = ((first_product * 10) % 11) % 10;
    if (first_remainder as u8) + b'0' != bytes[9] {
        return Err("Invalid first checksum digit");
    }

    let second_product = (block * CHECKSUM_MASK_2ND).reduce_sum();
    let second_remainder = ((second_product * 10) % 11) % 10;
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
