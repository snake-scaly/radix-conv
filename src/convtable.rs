// Copyright (c) 2015 Sergey "SnakE" Gromov
//
// See the file license.txt for copying permission.

use num::BigInt;
use crate::table::{Table, Row};

use num::bigint::ToBigInt;
use num::integer::Integer;
use num::traits::{One, Signed, ToPrimitive, Zero};

/// A table containing results of number conversion.
pub struct ConvTable {
    t: Table,
}

impl ConvTable {
    /// Creates an empty `ConvTable`.
    pub fn new() -> ConvTable {
        ConvTable { t: Table::new() }
    }

    /// Adds a line with conversions for a number.
    /// # Arguments
    /// * `orig` - the number as entered by the user
    /// * `num`  - the same number parsed into a `BigInt`
    pub fn push_result(&mut self, orig: &str, num: &BigInt) {
        let mut row = Row::new();
        row.push(&(orig.to_string() + ":"));
        row.push(&format!("{}", num));
        row.push(&format_hex(num));
        row.push(&format_bin(num));
        self.t.push(row);
    }

    /// Adds a line with a parse error.
    /// # Arguments
    /// * `orig`  - the number as entered by the user
    /// * `error` - associated error message
    pub fn push_error(&mut self, orig: &str, error: &str) {
        let mut row = Row::new();
        row.push(&(orig.to_string() + ":"));
        row.push_span(error);
        self.t.push(row);
    }

    /// Print the table contents.
    pub fn print(&self) {
        self.t.print();
    }
}

fn format_hex(n: &BigInt) -> String {
    const BASE: usize = 16;
    const POS: &'static [u8; BASE] = b"0123456789ABCDEF";
    const NEG: &'static [u8; BASE] = b"FEDCBA9876543210";

    let base = BASE.to_bigint().unwrap();
    let mut number = n.clone();
    let mut digits: Vec<char> = Vec::new();

    let hex = if number.is_negative() {
        number = -number - BigInt::one();
        NEG
    } else {
        POS
    };

    let mut correct_sign = false;
    loop {
        for _ in 0..2 {
            let (quot, rem) = number.div_rem(&base);
            number = quot;

            let d = rem.to_usize().unwrap();
            digits.push(hex[d] as char);

            correct_sign = d < 8;
        }

        if number.is_zero() && (!n.is_negative() || correct_sign) { break }
    }

    let digits: String = digits.into_iter().rev().collect();
    "0x".to_string() + &digits
}

fn format_bin(n: &BigInt) -> String {
    const POS: &'static [u8; 2] = b"01";
    const NEG: &'static [u8; 2] = b"10";

    let mut number = n.clone();
    let mut digits: Vec<char> = Vec::new();

    let bin = if number.is_negative() {
        number = -number - BigInt::one();
        NEG
    } else {
        POS
    };

    let mut correct_sign = false;
    loop {
        for _ in 0..8 {
            let d = if number.is_odd() { 1 } else { 0 };
            digits.push(bin[d] as char);
            number = number >> 1;
            correct_sign = d == 0;
        }
        if number.is_zero() && (!n.is_negative() || correct_sign) { break }
        digits.push('_');
    }

    let digits: String = digits.into_iter().rev().collect();
    "0b".to_string() + &digits
}

#[cfg(test)]
mod tests {
    use super::{format_hex, format_bin};
    use num::BigInt;
    use num::bigint::ToBigInt;

    #[test]
    fn format_positive_hex() {
        assert_eq!("0x01", format_hex(&1.to_bigint().unwrap()));
        assert_eq!("0x7F", format_hex(&127.to_bigint().unwrap()));
        assert_eq!("0x80", format_hex(&128.to_bigint().unwrap()));
        assert_eq!("0xFACE", format_hex(&64206.to_bigint().unwrap()));
    }

    #[test]
    fn format_negative_hex() {
        assert_eq!("0xFF", format_hex(&-1.to_bigint().unwrap()));
        assert_eq!("0x80", format_hex(&-128.to_bigint().unwrap()));
        assert_eq!("0xFF7F", format_hex(&-129.to_bigint().unwrap()));
        assert_eq!("0xFF0532", format_hex(&-64206.to_bigint().unwrap()));
    }

    #[test]
    fn format_positive_bin() {
        assert_eq!("0b00000001", format_bin(&1.to_bigint().unwrap()));
        assert_eq!("0b01111111", format_bin(&127.to_bigint().unwrap()));
        assert_eq!("0b10000000", format_bin(&128.to_bigint().unwrap()));
        assert_eq!("0b11111010_11001110", format_bin(&64206.to_bigint().unwrap()));
    }

    #[test]
    fn format_negative_bin() {
        assert_eq!("0b11111111", format_bin(&-1.to_bigint().unwrap()));
        assert_eq!("0b10000000", format_bin(&-128.to_bigint().unwrap()));
        assert_eq!("0b11111111_01111111", format_bin(&-129.to_bigint().unwrap()));
        assert_eq!("0b11111111_00000101_00110010", format_bin(&-64206.to_bigint().unwrap()));
    }
}
