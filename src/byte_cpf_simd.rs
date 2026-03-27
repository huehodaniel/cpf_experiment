use crate::byte_cpf::ByteCPFValidator;
use std::simd::prelude::*;

pub struct SimdByteCPFValidator;

impl ByteCPFValidator for SimdByteCPFValidator {
    fn new() -> Self {
        SimdByteCPFValidator {}
    }

    fn validate(&mut self, cpf: &[u8]) -> Result<(), &'static str> {
        validate_cpf(cpf)
    }
}

const CHECKSUM_MASK_1ST: Simd<u16, 16> =
    u16x16::from_array([10, 9, 8, 7, 6, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
const CHECKSUM_MASK_2ND: Simd<u16, 16> =
    u16x16::from_array([11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0]);

pub(crate) fn validate_cpf(cpf: &[u8]) -> Result<(), &'static str> {
    if cpf.len() != 11 {
        return Err("CPF must have 11 digits");
    }

    let block: Simd<u16, 16> = u8x16::load_or_default(cpf).cast();

    let first_product = (block * CHECKSUM_MASK_1ST).reduce_sum();
    let first_remainder = ((first_product * 10) % 11) % 10;
    if (first_remainder as u8) != cpf[9] {
        return Err("Invalid first checksum digit");
    }

    let second_product = (block * CHECKSUM_MASK_2ND).reduce_sum();
    let second_remainder = ((second_product * 10) % 11) % 10;
    if (second_remainder as u8) != cpf[10] {
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
        assert_eq!(
            validate_cpf(&as_byte_cpf("123")),
            Err("CPF must have 11 digits")
        );
        assert_eq!(
            validate_cpf(&as_byte_cpf("123456789012")),
            Err("CPF must have 11 digits")
        );
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
            assert!(
                validate_cpf(&as_byte_cpf(&cpf)).is_ok(),
                "Failed for CPF: {}",
                cpf
            );
            if rand::random_bool(0.00001f64) {
                println!("Current CPF: {}", CPF::new(cpf));
            }
        }
    }
}
