pub fn cpf_string_to_int(cpf: &str) -> Result<u64, &'static str> {
    if cpf.len() != 11 {
        return Err("Invalid CPF length");
    }

    let bytes = cpf.as_bytes();
    let d0 = (bytes[0] - b'0') as u64 * 10_000_000_000;
    let d1 = (bytes[1] - b'0') as u64 * 1_000_000_000;
    let d2 = (bytes[2] - b'0') as u64 * 100_000_000;
    let d3 = (bytes[3] - b'0') as u64 * 10_000_000;
    let d4 = (bytes[4] - b'0') as u64 * 1_000_000;
    let d5 = (bytes[5] - b'0') as u64 * 100_000;
    let d6 = (bytes[6] - b'0') as u64 * 10_000;
    let d7 = (bytes[7] - b'0') as u64 * 1_000;
    let d8 = (bytes[8] - b'0') as u64 * 100;
    let d9 = (bytes[9] - b'0') as u64 * 10;
    let d10 = (bytes[10] - b'0') as u64;
    let sum = d0 + d1 + d2 + d3 + d4 + d5 + d6 + d7 + d8 + d9 + d10;

    Ok(sum)
}

pub fn cpf_string_to_float(cpf: &str) -> Result<f64, &'static str> {
    cpf_string_to_int(cpf).map(|x| x as f64)
}

pub fn cpf_int_to_string(cpf: u64) -> Result<String, &'static str> {
    if cpf > 999_999_999_99 {
        return Err("CPF value exceeds maximum allowed");
    }

    let mut bytes = vec![0u8; 11];
    bytes[0] = ((cpf / 10_000_000_000) % 10) as u8 + b'0';
    bytes[1] = ((cpf / 1_000_000_000) % 10) as u8 + b'0';
    bytes[2] = ((cpf / 100_000_000) % 10) as u8 + b'0';
    bytes[3] = ((cpf / 10_000_000) % 10) as u8 + b'0';
    bytes[4] = ((cpf / 1_000_000) % 10) as u8 + b'0';
    bytes[5] = ((cpf / 100_000) % 10) as u8 + b'0';
    bytes[6] = ((cpf / 10_000) % 10) as u8 + b'0';
    bytes[7] = ((cpf / 1_000) % 10) as u8 + b'0';
    bytes[8] = ((cpf / 100) % 10) as u8 + b'0';
    bytes[9] = ((cpf / 10) % 10) as u8 + b'0';
    bytes[10] = (cpf % 10) as u8 + b'0';

    let result = unsafe { String::from_utf8_unchecked(bytes) };

    Ok(result)
}

pub fn cpf_int_to_string_fmt(cpf: u64) -> Result<String, &'static str> {
    if cpf > 999_999_999_99 {
        return Err("CPF value exceeds maximum allowed");
    }

    Ok(format!("{:011}", cpf))
}

pub fn cpf_string_to_byte(cpf: &str) -> Vec<u8> {
    cpf.as_bytes().iter().map(|&x| x - b'0').collect::<Vec<_>>()
}
