use crate::constants::control_line::ControlLine;

#[derive(Clone, Copy)]
pub struct ControlWord(u64);

impl ControlWord {
    pub fn from_lines(lines: &[ControlLine]) -> Self {
        let value = lines
            .iter()
            .map(|line| line.value())
            .fold(0, |acc, val| acc | val);
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl core::fmt::Debug for ControlWord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:0>38b}", self.0)
    }
}

impl core::ops::BitOr for ControlWord {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl From<Vec<ControlLine>> for ControlWord {
    fn from(lines: Vec<ControlLine>) -> Self {
        Self::from_lines(&lines)
    }
}

impl<const N: usize> From<[ControlLine; N]> for ControlWord {
    fn from(lines: [ControlLine; N]) -> Self {
        Self::from_lines(&lines)
    }
}
