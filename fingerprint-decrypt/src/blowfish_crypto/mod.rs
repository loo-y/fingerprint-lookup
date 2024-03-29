use blowfish::{Blowfish, cipher::BlockDecrypt};
use generic_array::GenericArray;
use getrandom::{getrandom, Error};
pub mod error;
pub mod util;

#[cfg(test)]
mod test;

pub fn encipher(plain_text: &str, key: &str) -> Result<String, error::Error> {
    let keyb = key.as_bytes();

    validate_key(keyb)?;

    let plain_textb = util::as_padded_bytes(plain_text);

    let bf = init_blowfish(keyb);

    let mut cipher_text_bytes: Vec<u8> = vec![];

    for block in plain_textb.chunks(8) {
        let lblock: [u8; 4] = block[0..4].try_into().expect("selected four bytes");
        let rblock: [u8; 4] = block[4..8].try_into().expect("selected four bytes");

         let [lenc_block, renc_block] = bf.bc_encrypt([ 
             u32::from_be_bytes(lblock), 
             u32::from_be_bytes(rblock) 
         ]); 

        cipher_text_bytes.extend_from_slice(&u32::to_be_bytes(lenc_block));
        cipher_text_bytes.extend_from_slice(&u32::to_be_bytes(renc_block));
    }

    let cipher_text = util::to_hex_string(&cipher_text_bytes);

    Ok(cipher_text)
}

pub fn decipher(cipher_text: &str, key: &str) -> Result<String, error::Error> {
    let keyb = key.as_bytes();

    if let Err(e) = validate_key(keyb) { return Err(e) }

    let encrypted_bytes = util::to_byte_vec(cipher_text);

    let bf = init_blowfish(keyb);

    let mut plain_text_fragments: Vec<String> = vec![];

    for block in encrypted_bytes.chunks(8) {
        let mut deciphered_block = GenericArray::from_slice(block).to_owned();

        bf.decrypt_block(&mut deciphered_block);

        let plain_text_fragment = deciphered_block.iter()
            .map(|b| (b.to_owned() as char).to_string())
            .collect::<Vec<String>>()
            .join("");

        plain_text_fragments.push(plain_text_fragment);
    }

    let plain_text = plain_text_fragments.join("").trim_end().to_string();

    Ok(plain_text)
}

fn init_blowfish(keyb: &[u8]) -> Blowfish {
    let mut bf = Blowfish::bc_init_state();
    bf.bc_expand_key(keyb);
    bf
}

fn validate_key(keyb: &[u8]) -> Result<(), error::Error> {
    let key_len = keyb.len();

    if key_len < 4 || key_len > 56 {
        return Err(error::Error::InvalidKeyLength);
    }

    Ok(())
}

pub fn transform_text(plain_text: &str) -> String {
    let mut result = String::new();

    let chars = plain_text.chars().take(8);
    for ch in chars {
        let transformed_ch = if ch.is_digit(10) {
            let digit = ch.to_digit(10).unwrap();
            (9 - digit).to_string().chars().next().unwrap()
        } else if ch.is_ascii_alphabetic() {
            let lowercase_ch = ch.to_ascii_lowercase();
            let index = (lowercase_ch as u8 - b'a') as u8;
            let desc_index = 25 - index;
            (b'a' + desc_index) as char
        } else {
            ch
        };

        result.push(transformed_ch);
    }

    result
}

pub fn twice_encrypt(plain_text: &str, key: Option<String>) -> Result<String, String> {
    let random_string = generate_random_string(8);
    let random_key = encipher(&random_string, &random_string).unwrap();
    // 取前8位
    let random_key = &random_key[0..8];
    // 翻转
    let random_key_transform = transform_text(&random_key);
    // base64
    let random_key_transform_base64 = base64::encode(random_key_transform);
    let first_encryption = match key {
        Some(value) => {
            encipher(plain_text, &value)
         }
        None => {

            encipher(plain_text, &random_key_transform_base64)
        }
    };

    if first_encryption.is_err() {
        return Err("first_encryption.is_err()".to_string())
    }
    let first_encryption = first_encryption.unwrap();
    let key_by_fisrt_encryption = &first_encryption[0..8];
    let key_transform = transform_text(&key_by_fisrt_encryption);
    let key_transform_base64 = base64::encode(key_transform);
    let combined_transform_base64 = String::new() + &random_key_transform_base64 + &key_transform_base64;
    let second_encryption = encipher(&first_encryption, &combined_transform_base64);
    if second_encryption.is_err() {
        return Err(key_by_fisrt_encryption.to_string());
    }
    let second_encryption = second_encryption.unwrap();
    let final_encryption = String::new() + &random_key + &key_by_fisrt_encryption + &second_encryption;
    Ok(final_encryption)
}

pub fn twice_decrypt(cipher_text: &str,  key: Option<String>) -> Result<String, String> {
    // 获取前8位random key
    let random_key = &cipher_text[0..8];
    // 翻转
    let random_key_transform = transform_text(&random_key);
    // base64
    let random_key_transform_base64 = base64::encode(random_key_transform);

    // 获取9-16位 key
    let key_by_fisrt_encryption: &str = &cipher_text[8..16];
    // 翻转 key
    let key_transform = transform_text(&key_by_fisrt_encryption);
    // base64
    let key_transform_base64 = base64::encode(key_transform);

    let combined_transform_base64 = String::new() + &random_key_transform_base64 + &key_transform_base64;

    let second_decryption = decipher(&cipher_text[16..], &combined_transform_base64);
    if second_decryption.is_err() {
        return Err(String::new())
    }
    let second_decryption = second_decryption.unwrap();
    let first_decryption = match key {
        Some(value) => {
            decipher(&second_decryption, &value)
         }
        None => {
            decipher(&second_decryption, &random_key_transform_base64)
        }
    };

    if first_decryption.is_err() {
        return Err(String::new())
    }

    Ok(first_decryption.unwrap())
}

fn generate_random_string(length: usize) -> String {
    let mut byte = [0u8; 1];
    let mut result = String::new();

    loop{
        if getrandom(&mut byte).is_err() {
            continue;
        }
        let digit = byte[0] % 36;
        let ch = if digit < 10 {
            (b'0' + digit) as char
        } else {
            (b'a' + (digit - 10)) as char
        };

        result.push(ch);
        if result.len() >= length {
            break;
        }
    }

    result
}