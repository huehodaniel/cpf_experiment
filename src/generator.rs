//#[derive(Copy, Clone)]
pub struct CPFGenerator {
    idx: u32,
}

impl CPFGenerator {
    pub fn new() -> Self {
        Self { idx: 0 }
    }

    #[allow(dead_code)]
    pub(crate) fn from_idx(idx: u32) -> Self {
        Self { idx }
    }

    // given a CPF without the verification digit, returns the CPF with the verification digit
    pub fn get_with_checksum(base: String) -> String {
        let mut first_checksum: i32 = 0;
        let bytes = base.as_bytes();
        let mut factor: i32 = 10;
        for x in bytes.iter() {
            first_checksum += (*x - b'0') as i32 * factor;
            factor -= 1;
        }

        let mut first_remainder = (first_checksum * 10) % 11;
        if first_remainder == 10 {
            first_remainder = 0;
        }

        let mut second_checksum: i32 = first_remainder * 2;
        factor = 11;
        for x in bytes.iter() {
            second_checksum += (*x - b'0') as i32 * factor;
            factor -= 1;
        }

        let mut second_remainder = (second_checksum * 10) % 11;
        if second_remainder == 10 {
            second_remainder = 0;
        }

        format!("{}{}{}", base, first_remainder, second_remainder)
    }

    // given a number, returns the CPF with the verification digit
    pub fn get_with_checksum_from_int(digits: u32) -> String {
        Self::get_with_checksum(format!("{:09}", digits))
    }
}

impl Iterator for CPFGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > 999_999_999 {
            return None;
        }
        let res = Self::get_with_checksum_from_int(self.idx);
        self.idx += 1;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_first_and_last() {
        let mut generator = CPFGenerator::new();
        assert_eq!(generator.next(), Some("00000000000".to_string()));

        let mut generator = CPFGenerator::from_idx(999_999_999);
        assert_eq!(generator.next(), Some("99999999999".to_string()));
        assert_eq!(generator.next(), None);
    }

    #[test]
    fn test_get_with_checksum() {
        // Test with a known valid CPF: 52998224725
        let base = "529982247".to_string();
        assert_eq!(CPFGenerator::get_with_checksum(base), "52998224725");
    }
}
