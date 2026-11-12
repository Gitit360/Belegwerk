//! Money without binary floats (D-22).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// ISO 4217 currency code (uppercase).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyCode(pub String);

impl CurrencyCode {
    pub fn eur() -> Self {
        Self("EUR".to_string())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MoneyError {
    #[error("currency mismatch: {0} vs {1}")]
    CurrencyMismatch(String, String),
}

/// Amount in minor units (e.g. cents for EUR).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoneyAmount {
    pub amount_minor: i64,
    pub currency: CurrencyCode,
}

impl MoneyAmount {
    pub fn new(amount_minor: i64, currency: CurrencyCode) -> Self {
        Self {
            amount_minor,
            currency,
        }
    }

    pub fn checked_add(&self, other: &Self) -> Result<Self, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(
                self.currency.0.clone(),
                other.currency.0.clone(),
            ));
        }
        Ok(Self {
            amount_minor: self
                .amount_minor
                .checked_add(other.amount_minor)
                .expect("amount overflow"),
            currency: self.currency.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_same_currency() {
        let a = MoneyAmount::new(100, CurrencyCode::eur());
        let b = MoneyAmount::new(50, CurrencyCode::eur());
        assert_eq!(a.checked_add(&b).unwrap().amount_minor, 150);
    }

    #[test]
    fn rejects_currency_mismatch() {
        let a = MoneyAmount::new(100, CurrencyCode::eur());
        let b = MoneyAmount::new(50, CurrencyCode("USD".into()));
        assert!(a.checked_add(&b).is_err());
    }
}
