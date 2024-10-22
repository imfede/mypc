use nom::bytes::complete::tag;
use nom::character::complete::hex_digit1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

pub fn hex_u8(input: &str) -> IResult<&str, u8> {
    // TODO: instead of hex_digit1, ensure that we only have one or two digits.
    map(tuple((tag("0x"), hex_digit1)), |(_, digits)| u8::from_str_radix(digits, 16).unwrap())(input)
}

pub fn hex_u16(input: &str) -> IResult<&str, u16> {
    // TODO: instead of hex_digit1, ensure that we only have one to four digits.
    map(tuple((tag("0x"), hex_digit1)), |(_, digits)| u16::from_str_radix(digits, 16).unwrap())(input)
}

#[cfg(test)]
mod tests {
    use crate::hex_u8::hex_u8;

    #[test]
    fn test_hex_u8() {
        assert_eq!(hex_u8("0x00"), Ok(("", 0)));
        assert_eq!(hex_u8("0x01"), Ok(("", 1)));
        assert_eq!(hex_u8("0x12"), Ok(("", 18)));
        assert_eq!(hex_u8("0xFF"), Ok(("", 255)));
    }
}