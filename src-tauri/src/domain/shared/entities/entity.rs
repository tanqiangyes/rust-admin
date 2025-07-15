use crate::domain::shared::value_objects::Id;
use serde::{Deserialize, Serialize};

/// 实体特征
pub trait Entity<T> {
    /// 获取实体ID
    fn id(&self) -> &Id<T>;
    
    /// 判断两个实体是否相等（基于ID）
    fn equals(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// 实体基类
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityBase<T> {
    pub id: Id<T>,
}

impl<T> EntityBase<T> {
    pub fn new(id: Id<T>) -> Self {
        Self { id }
    }
}

impl<T> Entity<T> for EntityBase<T> {
    fn id(&self) -> &Id<T> {
        &self.id
    }
} 