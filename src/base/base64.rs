pub fn decode(base64: &str) -> Option<Vec<u8>> {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut bits = 0u32;
    let mut acc = 0;
    let mut dec = Vec::with_capacity((base64.len() * 3) / 4);

    for c in base64.chars() {
        if c == '=' {
            break;
        }

        let value = CHARSET.iter().position(|&x| x == c as u8)? as u32;
        acc = (acc << 6) | value;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            dec.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }

    Some(dec)
}

