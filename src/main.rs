#![allow(dead_code)]

use hex;
use base64::prelude::*;
use num_bigint::BigUint;

fn main() {
    // Introduction challenges
    // ascii();
    // hexadecimal();
    // base_sixty_four();
    // bytes_and_big_integer();
    // xor_starter();
    // xor_properties();
    // favourite_byte();
    // you_either_know_xor_you_dont()
}

fn ascii() {
    let enc = [99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125];
    let flag = cryptohack::ascii_to_string(&enc);
    println!("{}", flag);
}

fn hexadecimal() {
    let enc = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let dec_bytes = hex::decode(enc)
        .unwrap();
    let flag = cryptohack::ascii_to_string(&dec_bytes);
    println!("{}", flag);
}

fn base_sixty_four() {
    let enc = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let hex_dec_bytes = hex::decode(enc)
        .unwrap();
    let base64_enc_flag = BASE64_STANDARD.encode(hex_dec_bytes);
    println!("{}", base64_enc_flag);
}

fn bytes_and_big_integer() {
    let big_number = b"11515195063862318899931685488813747395775516287289682636499965282714637259206269";
    let hex_value = format!("{:x}", BigUint::parse_bytes(big_number, 10).unwrap());
    let dec_bytes = hex::decode(hex_value)
        .unwrap();
    let flag = cryptohack::ascii_to_string(&dec_bytes);
    println!("{}", flag);
}

fn xor_starter() {
    let msg = "label";
    let xored_bytes = cryptohack::xor(msg.as_bytes(), &[13]);
    println!("crypto{{{}}}", cryptohack::ascii_to_string(&xored_bytes))
}

fn xor_properties() {
    let key1 = BigUint::parse_bytes(b"a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313", 16).unwrap();
    let key23 = BigUint::parse_bytes(b"c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1", 16).unwrap();
    let flag123 = BigUint::parse_bytes(b"04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf", 16).unwrap();
    let flag = flag123 ^ key1 ^ key23;
    println!("{}", cryptohack::bignum_to_string(flag));
}

fn favourite_byte() {
    let enc = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d";
    let hex_decoded = hex::decode(&enc)
        .unwrap();
    let key = hex_decoded[0] ^ 'c' as u8;
    let flag_bytes = cryptohack::xor(&hex_decoded, &[key]);
    println!("{}", cryptohack::ascii_to_string(&flag_bytes));
}

fn you_either_know_xor_you_dont() {
    let enc = "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104";
    let hex_decoded = hex::decode(enc)
        .unwrap();
    // println!("len: {}", hex_decoded.len());
    // for (x, y) in "crypto{".chars().enumerate() {
    //     let key =  hex_decoded[x] ^ y as u8;
    //     println!("{}", key);
    // }
    // let ascii = [109, 121, 88, 79, 82, 107, 101];
    // println!("{}", cryptohack::ascii_to_string(&ascii));
    let key = b"myXORkey";
    let flag_bytes = cryptohack::xor(&hex_decoded, key);
    println!("{}", cryptohack::ascii_to_string(&flag_bytes));
}