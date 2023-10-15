pub fn decode(base58: &str) -> Option<Vec<u8>> {
    const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    if base58.is_empty() {
        return Some(Vec::new());
    }

    let mut result = vec![0u8; base58.len()]; // 初始化一个足够大的缓冲区

    for c in base58.chars() {
        let mut carry = match ALPHABET.find(c) {
            Some(x) => x as u32,
            None => return None, // 非 Base58 字符
        };

        for byte in result.iter_mut().rev() {
            carry += (*byte & 0xff) as u32 * 58; // 用 58 倍的当前字节值加上 carry
            *byte = carry as u8; // 存储新的字节值
            carry >>= 8; // 删除已经存储的部分，保留溢出
        }
    }

    // 移除前导零
    let leading_zeros = base58.chars().take_while(|&c| c == '1').count();
    let result = &result[result.len() - leading_zeros - base58.len() + leading_zeros..];

    Some(result.to_vec())
}

