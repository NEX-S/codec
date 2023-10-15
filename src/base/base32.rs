pub fn decode(base32: &str) -> Option<Vec<u8>> {
    const PADDING: char = '=';

    let mut bits = 0u32;
    let mut acc = 0;
    let mut dec = Vec::with_capacity((base32.len() * 5) / 8);

    for c in base32.chars() {
        if c == PADDING {
            break;
        }

        bits += 5;

        let value = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            '2'..='7' => c as u32 - '2' as u32 + 26,
            _ => return None,
        };

        acc <<= 5;
        acc |= value;

        if bits >= 8 {
            bits -= 8;
            dec.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }

    Some(dec)
}
