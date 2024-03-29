use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};

const U128_LEFTMOST_BIT: u128 = 170141183460469231731687303715884105728;

#[derive(Clone, Copy, Debug)]
pub struct U256 {
    left: u128,
    right: u128,
}

impl U256 {
    pub const ONE: Self = Self { left: 0, right: 1 };
    pub const LEFTMOST_BIT: Self = Self {
        left: U128_LEFTMOST_BIT,
        right: 0,
    };

    pub fn new() -> Self {
        Self { left: 0, right: 0 }
    }

    pub fn from_u128(val: u128) -> Self {
        Self {
            left: 0,
            right: val,
        }
    }

    pub fn as_usize(&self) -> usize {
        if self.left != 0 {
            panic!("Too big for usize: {:?}", self)
        }
        self.right as usize
    }
}

impl Shl<usize> for U256 {
    type Output = Self;

    fn shl(self, bits: usize) -> Self::Output {
        let mut out = self.clone();
        out.shl_assign(bits);
        out
    }
}

impl ShlAssign<usize> for U256 {
    fn shl_assign(&mut self, bits: usize) {
        if bits >= 128 {
            // replace left side with right side, and apply remaining shift
            let rem = bits - 128;
            self.left = self.right << rem;
            self.right = 0;
        } else {
            // move some bits from right to left
            let mask = self.right >> (128 - bits);
            self.left <<= bits;
            self.right <<= bits;
            self.left |= mask;
        }
    }
}

impl Shr<usize> for U256 {
    type Output = Self;

    fn shr(self, bits: usize) -> Self::Output {
        let mut out = self.clone();
        out.shr_assign(bits);
        out
    }
}

impl ShrAssign<usize> for U256 {
    fn shr_assign(&mut self, bits: usize) {
        if bits >= 128 {
            // replace right side with left side, and apply remaining shift
            let rem = bits - 128;
            self.right = self.left >> rem;
            self.left = 0;
        } else {
            // move some bits from left to right
            let mask = self.left << (128 - bits);
            self.left >>= bits;
            self.right >>= bits;
            self.right |= mask;
        }
    }
}

impl BitAnd for U256 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let mut out = self.clone();
        out.bitand_assign(other);
        out
    }
}

impl BitAndAssign for U256 {
    fn bitand_assign(&mut self, other: Self) {
        self.left &= other.left;
        self.right &= other.right;
    }
}

impl BitOr for U256 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let mut out = self.clone();
        out.bitor_assign(other);
        out
    }
}

impl BitOrAssign for U256 {
    fn bitor_assign(&mut self, other: Self) {
        self.left |= other.left;
        self.right |= other.right;
    }
}

impl PartialEq<U256> for U256 {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl PartialEq<u128> for U256 {
    fn eq(&self, val: &u128) -> bool {
        self.left == 0 && self.right == *val
    }
}

impl std::fmt::Display for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", u128_to_s(self.left), u128_to_s(self.right),)
    }
}

pub fn u128_to_s(num: u128) -> String {
    let mut n = num;
    (0..128)
        .map(|_| {
            let b = n & U128_LEFTMOST_BIT;
            n <<= 1;
            match b {
                U128_LEFTMOST_BIT => '#',
                0 => '.',
                _ => panic!("Unexpected value: {:?}", b),
            }
        })
        .collect()
}

pub fn u8_to_s(num: u8) -> String {
    let mut n = num;
    (0..8)
        .map(|_| {
            let b = n & 128;
            n <<= 1;
            match b {
                128 => '#',
                0 => '.',
                _ => panic!("Unexpected value: {:?}", b),
            }
        })
        .collect()
}
