use sha2::Digest;

pub fn hex_to_binary(hex_string: &str) -> String {
    if hex_string.len() % 2 != 0 {
        panic!("Hexadecimal string must have an even number of characters")
    }

    let binary: Option<String> = (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i+2], 16).ok())
        .map(|byte| byte.map(|b| format!("{:08b}", b)))
        .collect();

    binary.unwrap()
}

pub fn sha256_hash(data: &String) -> String {
    sha2::Sha256::digest(&data.as_bytes())
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
