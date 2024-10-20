#[allow(unused)]
pub trait StartsWith {
    fn starts_with(&self, prefix: Self, prefix_bits: usize) -> bool;
    fn binary_representation_starts_with(&self, prefix: &'static str) -> bool;
}

impl StartsWith for u8 {
    fn starts_with(&self, prefix: Self, prefix_bits: usize) -> bool {
        let mask = 0b11_11_11_11 << ((Self::BITS as usize) - prefix_bits);
        let prefix_aligned = prefix << ((Self::BITS as usize) - prefix_bits);
        let masked = self & mask;
        masked == prefix_aligned
    }

    fn binary_representation_starts_with(&self, prefix_str: &'static str) -> bool {
        let cleaned = prefix_str.replace("_", "").to_ascii_lowercase();
        let prefix = u8::from_str_radix(&cleaned, 2).unwrap();
        self.starts_with(prefix, cleaned.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with() {
        let value: u8 = 0b00_10_11_11;
        assert!(value.starts_with(0b00_10, 4));
        assert!(value.starts_with(0b00, 2));
        assert!(!value.starts_with(0b01, 2));
    }

    #[test]
    fn starts_with_str() {
        let value: u8 = 0b00_10_11_11;
        assert!(value.binary_representation_starts_with("0010"));
        assert!(value.binary_representation_starts_with("00"));
        assert!(!value.binary_representation_starts_with("01"));
        assert!(value.binary_representation_starts_with("00_10"));
        assert!(value.binary_representation_starts_with("00_10_11"));
    }
}