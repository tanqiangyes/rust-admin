//! 金钱值对象

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use crate::domain::shared::errors::DomainError;

/// 金钱值对象，处理货币相关的业务逻辑
#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Money {
    /// 金额（以最小单位存储，如分）
    amount_cents: i64,
    /// 货币代码 (ISO 4217)
    currency: Currency,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    CNY, // 人民币
    USD, // 美元
    EUR, // 欧元
    JPY, // 日元
}

impl Money {
    /// 创建新的金钱对象（金额以元为单位）
    pub fn new(amount: f64, currency: Currency) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::InvalidMoney("金额不能为负数".to_string()));
        }

        if amount > 1_000_000_000.0 {
            return Err(DomainError::InvalidMoney("金额超过最大限制".to_string()));
        }

        // 转换为分（最小单位）
        let amount_cents = (amount * 100.0).round() as i64;

        Ok(Self {
            amount_cents,
            currency,
        })
    }

    /// 创建人民币
    pub fn cny(amount: f64) -> Result<Self, DomainError> {
        Self::new(amount, Currency::CNY)
    }

    /// 创建美元
    pub fn usd(amount: f64) -> Result<Self, DomainError> {
        Self::new(amount, Currency::USD)
    }

    /// 创建零金额
    pub fn zero(currency: Currency) -> Self {
        Self {
            amount_cents: 0,
            currency,
        }
    }

    /// 获取金额（以元为单位）
    pub fn amount(&self) -> f64 {
        self.amount_cents as f64 / 100.0
    }

    /// 获取金额（以分为单位）
    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    /// 获取货币
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// 加法运算
    pub fn add(&self, other: &Money) -> Result<Money, DomainError> {
        self.ensure_same_currency(other)?;
        
        let new_amount_cents = self.amount_cents + other.amount_cents;
        if new_amount_cents < 0 {
            return Err(DomainError::InvalidMoney("运算结果为负数".to_string()));
        }

        Ok(Self {
            amount_cents: new_amount_cents,
            currency: self.currency.clone(),
        })
    }

    /// 减法运算
    pub fn subtract(&self, other: &Money) -> Result<Money, DomainError> {
        self.ensure_same_currency(other)?;
        
        let new_amount_cents = self.amount_cents - other.amount_cents;
        if new_amount_cents < 0 {
            return Err(DomainError::InvalidMoney("余额不足".to_string()));
        }

        Ok(Self {
            amount_cents: new_amount_cents,
            currency: self.currency.clone(),
        })
    }

    /// 乘法运算
    pub fn multiply(&self, factor: f64) -> Result<Money, DomainError> {
        if factor < 0.0 {
            return Err(DomainError::InvalidMoney("乘数不能为负数".to_string()));
        }

        let new_amount_cents = (self.amount_cents as f64 * factor).round() as i64;

        Ok(Self {
            amount_cents: new_amount_cents,
            currency: self.currency.clone(),
        })
    }

    /// 检查是否为零
    pub fn is_zero(&self) -> bool {
        self.amount_cents == 0
    }

    /// 检查是否为正数
    pub fn is_positive(&self) -> bool {
        self.amount_cents > 0
    }

    /// 比较大小
    pub fn is_greater_than(&self, other: &Money) -> Result<bool, DomainError> {
        self.ensure_same_currency(other)?;
        Ok(self.amount_cents > other.amount_cents)
    }

    /// 比较大小
    pub fn is_less_than(&self, other: &Money) -> Result<bool, DomainError> {
        self.ensure_same_currency(other)?;
        Ok(self.amount_cents < other.amount_cents)
    }

    /// 确保货币相同
    fn ensure_same_currency(&self, other: &Money) -> Result<(), DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::InvalidMoney("货币类型不匹配".to_string()));
        }
        Ok(())
    }
}

impl Debug for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Money({:.2} {:?})", self.amount(), self.currency)
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self.currency {
            Currency::CNY => "¥",
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::JPY => "¥",
        };
        write!(f, "{}{:.2}", symbol, self.amount())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_money_with_valid_amount() {
        let money = Money::cny(99.99).unwrap();
        assert_eq!(money.amount(), 99.99);
        assert_eq!(money.amount_cents(), 9999);
    }

    #[test]
    fn should_reject_negative_amount() {
        let result = Money::cny(-10.0);
        assert!(result.is_err());
    }

    #[test]
    fn should_add_same_currency() {
        let m1 = Money::cny(10.50).unwrap();
        let m2 = Money::cny(5.25).unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.amount(), 15.75);
    }

    #[test]
    fn should_fail_add_different_currency() {
        let m1 = Money::cny(10.0).unwrap();
        let m2 = Money::usd(5.0).unwrap();
        let result = m1.add(&m2);
        assert!(result.is_err());
    }

    #[test]
    fn should_multiply_by_factor() {
        let money = Money::cny(10.0).unwrap();
        let result = money.multiply(2.5).unwrap();
        assert_eq!(result.amount(), 25.0);
    }

    #[test]
    fn should_compare_amounts() {
        let m1 = Money::cny(10.0).unwrap();
        let m2 = Money::cny(5.0).unwrap();
        
        assert!(m1.is_greater_than(&m2).unwrap());
        assert!(m2.is_less_than(&m1).unwrap());
    }
} 