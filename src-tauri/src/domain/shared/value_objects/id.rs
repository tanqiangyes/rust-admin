//! 类型安全的ID值对象

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use uuid::Uuid;
use crate::domain::shared::errors::DomainError;

/// 通用ID值对象，使用幻影类型确保类型安全
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id<T> {
    value: String,
    #[serde(skip)]
    _marker: std::marker::PhantomData<T>,
}

impl<T> Id<T> {
    /// 生成新的ID
    pub fn generate() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
            _marker: std::marker::PhantomData,
        }
    }

    /// 从字符串创建ID
    pub fn from_string(value: String) -> Result<Self, DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::InvalidId("ID不能为空".to_string()));
        }

        // 验证UUID格式
        if Uuid::parse_str(&value).is_err() {
            return Err(DomainError::InvalidId("无效的ID格式".to_string()));
        }

        Ok(Self {
            value,
            _marker: std::marker::PhantomData,
        })
    }

    /// 获取ID值
    pub fn value(&self) -> &str {
        &self.value
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.value)
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::generate()
    }
}

// 领域实体类型标记
pub struct UserEntity;
pub struct ProductEntity;
pub struct CategoryEntity;
pub struct OrderEntity;
pub struct RoleEntity;
pub struct ConfigurationEntity;

// 具体的类型安全ID
pub type UserId = Id<UserEntity>;
pub type ProductId = Id<ProductEntity>;
pub type CategoryId = Id<CategoryEntity>;
pub type OrderId = Id<OrderEntity>;
pub type RoleId = Id<RoleEntity>;
pub type ConfigurationId = Id<ConfigurationEntity>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_different_ids() {
        let id1: UserId = UserId::generate();
        let id2: UserId = UserId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn should_create_id_from_valid_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = UserId::from_string(uuid_str.to_string());
        assert!(id.is_ok());
        assert_eq!(id.unwrap().value(), uuid_str);
    }

    #[test]
    fn should_reject_empty_id() {
        let id = UserId::from_string("".to_string());
        assert!(id.is_err());
    }

    #[test]
    fn should_reject_invalid_uuid() {
        let id = UserId::from_string("invalid-uuid".to_string());
        assert!(id.is_err());
    }
} 