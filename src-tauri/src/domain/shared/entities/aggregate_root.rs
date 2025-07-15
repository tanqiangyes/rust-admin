use super::Entity;
use crate::domain::shared::events::DomainEvent;
use crate::domain::shared::value_objects::Id;
use serde::{Deserialize, Serialize};

/// 聚合根特征
pub trait AggregateRoot<T>: Entity<T> {
    /// 获取未提交的事件
    fn uncommitted_events(&self) -> &Vec<Box<dyn DomainEvent>>;
    
    /// 标记事件为已提交
    fn mark_events_as_committed(&mut self);
    
    /// 添加领域事件
    fn add_event(&mut self, event: Box<dyn DomainEvent>);
}

/// 聚合根基类
#[derive(Debug)]
pub struct AggregateRootBase<T> {
    pub id: Id<T>,
    #[serde(skip)]
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

impl<T> AggregateRootBase<T> {
    pub fn new(id: Id<T>) -> Self {
        Self {
            id,
            uncommitted_events: Vec::new(),
        }
    }

    pub fn from_history(id: Id<T>, events: Vec<Box<dyn DomainEvent>>) -> Self {
        Self {
            id,
            uncommitted_events: events,
        }
    }
}

impl<T> Entity<T> for AggregateRootBase<T> {
    fn id(&self) -> &Id<T> {
        &self.id
    }
}

impl<T> AggregateRoot<T> for AggregateRootBase<T> {
    fn uncommitted_events(&self) -> &Vec<Box<dyn DomainEvent>> {
        &self.uncommitted_events
    }

    fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    fn add_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push(event);
    }
}

// 手动实现Clone，因为DomainEvent不能自动derive Clone
impl<T> Clone for AggregateRootBase<T> 
where 
    T: Clone 
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            uncommitted_events: Vec::new(), // 克隆时不复制事件
        }
    }
} 