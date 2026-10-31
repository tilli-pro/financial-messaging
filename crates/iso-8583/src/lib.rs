//! A spec-driven ISO 8583 codec.
//!
//! ISO 8583 messages are a message type indicator (MTI), one to three
//! bitmaps, and the data elements those bitmaps announce. How each data
//! element is encoded (length prefix, character set, packed BCD, binary,
//! composite subfields, TLV) is not fixed by the standard: every network and
//! acquirer host publishes its own variant. This crate therefore treats the
//! field layout as **data**: a field table (JSON) describes each data element,
//! and the codec is driven by that table.
//!
//! Generic ISO 8583:1987, 1993 and 2003 field tables will ship with the
//! crate. Network-specific tables (Visa, Mastercard, acquirer hosts) are
//! loaded by users and never shipped here.
//!
//! This crate is a codec only: no transport, no session management, no MAC
//! computation. Those belong to the connector that uses it.
//!
//! # Status
//!
//! Pre-release. Only the [`Mti`] type exists today; bitmaps, data elements,
//! field tables and encodings are on the roadmap. Nothing has been published
//! to crates.io.
//!
//! # Example
//!
//! ```
//! use iso_8583::Mti;
//!
//! let mti = Mti::parse("0200")?;
//! assert_eq!(mti.version(), 0); // ISO 8583:1987
//! assert_eq!(mti.class(), 2); // financial message
//! assert_eq!(mti.function(), 0); // request
//! assert_eq!(mti.origin(), 0); // acquirer
//! assert_eq!(mti.to_string(), "0200");
//! # Ok::<(), iso_8583::MtiError>(())
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use core::fmt;

/// An ISO 8583 message type indicator: four decimal digits.
///
/// The digits are, in order: the version of the standard, the message class,
/// the message function and the message origin. The meaning of each value is
/// defined by the standard and refined by each host; this type only
/// guarantees the shape (exactly four ASCII digits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mti(u16);

/// Why a string is not a valid message type indicator.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MtiError {
    /// The input is not exactly four bytes long.
    #[error("message type indicator must be 4 digits, got {0} bytes")]
    Length(usize),
    /// The input contains a byte that is not an ASCII digit.
    #[error("message type indicator has a non-digit byte 0x{byte:02x} at position {position}")]
    NonDigit {
        /// Zero-based position of the offending byte.
        position: usize,
        /// The offending byte.
        byte: u8,
    },
}

impl Mti {
    /// Parses a message type indicator from exactly four ASCII digits.
    ///
    /// # Errors
    ///
    /// Returns [`MtiError::Length`] if the input is not four bytes long and
    /// [`MtiError::NonDigit`] if any byte is not an ASCII digit (`0`-`9`).
    pub fn parse(input: &str) -> Result<Self, MtiError> {
        let bytes = input.as_bytes();
        if bytes.len() != 4 {
            return Err(MtiError::Length(bytes.len()));
        }
        let mut value: u16 = 0;
        for (position, &byte) in bytes.iter().enumerate() {
            if !byte.is_ascii_digit() {
                return Err(MtiError::NonDigit { position, byte });
            }
            value = value * 10 + u16::from(byte - b'0');
        }
        Ok(Self(value))
    }

    /// The first digit: version of ISO 8583 (0 = 1987, 1 = 1993, 2 = 2003).
    #[must_use]
    pub const fn version(self) -> u8 {
        self.digit(1000)
    }

    /// The second digit: message class (for example 1 = authorization,
    /// 2 = financial, 4 = reversal, 8 = network management).
    #[must_use]
    pub const fn class(self) -> u8 {
        self.digit(100)
    }

    /// The third digit: message function (for example 0 = request,
    /// 1 = request response, 2 = advice).
    #[must_use]
    pub const fn function(self) -> u8 {
        self.digit(10)
    }

    /// The fourth digit: message origin (for example 0 = acquirer,
    /// 2 = issuer).
    #[must_use]
    pub const fn origin(self) -> u8 {
        self.digit(1)
    }

    // A value modulo 10 always fits in u8.
    #[allow(clippy::cast_possible_truncation)]
    const fn digit(self, place: u16) -> u8 {
        ((self.0 / place) % 10) as u8
    }
}

impl fmt::Display for Mti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parses_digits() {
        let mti = Mti::parse("1430").unwrap();
        assert_eq!(
            (mti.version(), mti.class(), mti.function(), mti.origin()),
            (1, 4, 3, 0)
        );
    }

    #[test]
    fn keeps_leading_zeros() {
        assert_eq!(Mti::parse("0010").unwrap().to_string(), "0010");
        assert_eq!(Mti::parse("0000").unwrap().to_string(), "0000");
    }

    #[test]
    fn rejects_wrong_length() {
        assert_eq!(Mti::parse(""), Err(MtiError::Length(0)));
        assert_eq!(Mti::parse("200"), Err(MtiError::Length(3)));
        assert_eq!(Mti::parse("02000"), Err(MtiError::Length(5)));
    }

    #[test]
    fn rejects_non_digits() {
        assert_eq!(
            Mti::parse("02a0"),
            Err(MtiError::NonDigit {
                position: 2,
                byte: b'a'
            })
        );
        assert_eq!(
            Mti::parse("+200"),
            Err(MtiError::NonDigit {
                position: 0,
                byte: b'+'
            })
        );
    }

    #[test]
    fn rejects_non_ascii_digits() {
        // U+0663 ARABIC-INDIC DIGIT THREE is a digit, but not an ASCII one.
        assert!(Mti::parse("\u{663}00").is_err());
    }

    proptest! {
        #[test]
        fn string_round_trip(s in "[0-9]{4}") {
            prop_assert_eq!(Mti::parse(&s).unwrap().to_string(), s);
        }

        #[test]
        fn value_round_trip(x in 0u16..=9999) {
            let mti = Mti::parse(&format!("{x:04}")).unwrap();
            prop_assert_eq!(Mti::parse(&mti.to_string()).unwrap(), mti);
        }
    }
}
