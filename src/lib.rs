//! A simple dollar value representation; nothing more, nothing less.
//!
//! See [`Dollars`] below.

use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

/// A dollar value, backed by a single integer value in cents.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dollars {
    cent_value: i64,
}

impl Dollars {
    /// The dollars portion of the value.
    pub fn dollars(&self) -> i64 {
        (self.cent_value / 100).abs()
    }

    /// The cents portion of the value.
    pub fn cents(&self) -> i64 {
        (self.cent_value % 100).abs()
    }

    /// The value in cents.
    ///
    /// Note the difference between this method and [`cents`](Dollars::cents).
    pub fn in_cents(&self) -> i64 {
        self.cent_value
    }

    /// Whether or not the value is positive.
    pub fn is_positive(&self) -> bool {
        self.in_cents() > 0
    }
}

impl Add for Dollars {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::from(self.in_cents() + other.in_cents())
    }
}

impl Debug for Dollars {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Dollars {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Delegating the actual formatting to format!
        // so that it plays nice with custom format
        // specifiers. This seems stupid
        let as_str = format!(
            "{}${}.{:02}",
            if self.in_cents() < 0 { "-" } else { "" },
            self.dollars(),
            self.cents(),
        );

        Display::fmt(&as_str, f)
    }
}

impl From<i64> for Dollars {
    fn from(cent_value: i64) -> Self {
        Self { cent_value }
    }
}

impl FromStr for Dollars {
    type Err = ParseError;

    // TODO: document the somewhat wonky overflow handling + the fact that it's maybe slightly
    // more permissive than it should be
    fn from_str(s: &str) -> Result<Self, ParseError> {
        // there may be a +/- in front for sign
        // there may be $ in front of the value
        // the value may be an integer
        // if it specifies cents, the cents value must be two digits long
        if !s.is_ascii() {
            return Err(ParseErrorKind::NonAscii.into());
        }

        let mut chars = s.chars().peekable();
        let sign = match chars.peek().copied() {
            Some('-') => {
                chars.next();
                -1
            },

            c => {
                if c == Some('+') {
                    chars.next();
                }

                1
            },
        };

        if let Some('$') = chars.peek().copied() {
            chars.next();
        }

        let dollars = chars
            .by_ref()
            .take_while(|&c| c != '.')
            .try_fold(0_i64, |acc, c| {
                c.to_digit(10)
                    .ok_or(ParseErrorKind::InvalidDigit(c))
                    .and_then(|d| acc.checked_add(d as i64).ok_or(ParseErrorKind::Overflow))
            })?;
        let cents = match (chars.next(), chars.next()) {
            (Some('.'), _) | (_, Some('.')) => return Err(ParseErrorKind::ExtraDecimalPoint.into()),
            (Some(_), None) => return Err(ParseErrorKind::BadCentsLength.into()),
            (None, _) => 0,
            (Some(c1), Some(c2)) => {
                let d1 = c1.to_digit(10).ok_or(ParseErrorKind::InvalidDigit(c1))? as i64;
                let d2 = c2.to_digit(10).ok_or(ParseErrorKind::InvalidDigit(c1))? as i64;

                d1 * 10 + d2
            },
        };

        dollars
            .checked_mul(100)
            .and_then(|d| d.checked_add(cents))
            .and_then(|d| d.checked_mul(sign))
            .map(Self::from)
            .ok_or(ParseErrorKind::Overflow.into())
    }
}

impl Neg for Dollars {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            cent_value: -self.cent_value,
        }
    }
}

impl Sub for Dollars {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::from(self.in_cents() - other.in_cents())
    }
}

/// Opaque error capturing a failure to parse a [`Dollars`] from a string.
///
/// Note that the exact failure modes for parsing are not exposed directly.
#[derive(Clone, Debug, thiserror::Error)]
#[error("failed to parse dollars: {0}")]
pub struct ParseError(#[from] ParseErrorKind);

#[derive(Clone, Debug, thiserror::Error)]
enum ParseErrorKind {
    #[error("invalid digit '{0}'")]
    InvalidDigit(char),

    #[error("value overflows")]
    Overflow,

    #[error("cents must be two digits long")]
    BadCentsLength,

    #[error("too many decimal points")]
    ExtraDecimalPoint,

    #[error("non-ASCII strings are not allowed")]
    NonAscii,
}

#[cfg(test)]
mod tests {
    // sanity check the entire api
    // for printing:
    // for parsing:
    // sanity check some normal cases:
    // valid edge cases:
    // invalid edge cases:
    // slightly over-permissive cases:
    // weird overflow cases:
}
