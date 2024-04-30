use num_bigint::BigUint;

pub fn ascii_to_string(ords: &[u8]) -> String {
    ords
        .iter()
        .map(|x| *x as char)
        .collect::<String>()
}

pub fn xor(message: &[u8], key: &[u8]) -> Vec<u8> {
    message
        .iter()
        .enumerate()
        .map(|(x, y)| {
            *y ^ key[x % key.len()]
        })
        .collect::<Vec<u8>>()
}

pub fn bignum_to_string(number: BigUint) -> String {
    let dec_bytes = hex::decode(format!("{:x}", number))
        .unwrap();
    ascii_to_string(&dec_bytes)
}