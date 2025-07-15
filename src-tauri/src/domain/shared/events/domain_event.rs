//! 领域事件定义

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::domain::shared::value_objects::Timestamp;

/// 领域事件特征
#[async_trait]
pub trait DomainEvent: Send + Sync + std::fmt::Debug {
    /// 事件名称
    fn event_name(&self) -> &'static str;
    
    /// 事件发生时间
    fn occurred_at(&self) -> Timestamp;
    
    /// 事件数据（用于序列化）
    fn event_data(&self) -> serde_json::Value;
    
    /// 聚合根ID
    fn aggregate_id(&self) -> String;
    
    /// 事件版本（用于事件溯源）
    fn version(&self) -> u32 {
        1
    }
}

/// 基础领域事件实现
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseDomainEvent {
    pub event_id: String,
    pub event_name: String,
    pub aggregate_id: String,
    pub occurred_at: Timestamp,
    pub event_data: serde_json::Value,
    pub version: u32,
}

impl BaseDomainEvent {
    pub fn new(
        event_name: String,
        aggregate_id: String,
        event_data: serde_json::Value,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_name,
            aggregate_id,
            occurred_at: Timestamp::now(),
            event_data,
            version: 1,
        }
    }
}

#[async_trait]
impl DomainEvent for BaseDomainEvent {
    fn event_name(&self) -> &'static str {
        // 由于trait要求返回&'static str，但我们有动态字符串
        // 这里使用泄露的方式，实际使用中应该为每个事件定义静态常量
        Box::leak(self.event_name.clone().into_boxed_str())
    }

    fn occurred_at(&self) -> Timestamp {
        self.occurred_at.clone()
    }

    fn event_data(&self) -> serde_json::Value {
        self.event_data.clone()
    }

    fn aggregate_id(&self) -> String {
        self.aggregate_id.clone()
    }

    fn version(&self) -> u32 {
        self.version
    }
}

/// 事件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub user_id: Option<String>,
    pub source: String,
}

impl Default for EventMetadata {
    fn default() -> Self {
        Self {
            correlation_id: None,
            causation_id: None,
            user_id: None,
            source: "rust-admin".to_string(),
        }
    }
} 