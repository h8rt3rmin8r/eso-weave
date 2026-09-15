pub(crate) fn hex_lower(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        result.push(HEX[(byte >> 4) as usize] as char);
        result.push(HEX[(byte & 0x0f) as usize] as char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::hex_lower;

    #[test]
    fn lower_hex_preserves_leading_zeroes_and_byte_order() {
        assert_eq!(hex_lower([0x00, 0x0f, 0x10, 0xab, 0xff]), "000f10abff");
    }
}
