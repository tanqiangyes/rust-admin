use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// 领域错误枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainError {
    /// 验证错误
    Validation(String),
    /// 业务规则违反
    BusinessRule(String),
    /// 实体未找到
    NotFound(String),
    /// 权限不足
    Unauthorized(String),
    /// 资源冲突
    Conflict(String),
    /// 内部错误
    Internal(String),
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DomainError::Validation(msg) => write!(f, "验证错误: {}", msg),
            DomainError::BusinessRule(msg) => write!(f, "业务规则错误: {}", msg),
            DomainError::NotFound(msg) => write!(f, "未找到: {}", msg),
            DomainError::Unauthorized(msg) => write!(f, "权限不足: {}", msg),
            DomainError::Conflict(msg) => write!(f, "冲突: {}", msg),
            DomainError::Internal(msg) => write!(f, "内部错误: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

/// 领域结果类型
pub type DomainResult<T> = Result<T, DomainError>; 