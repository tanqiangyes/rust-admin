//! 事件发布器

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::shared::{events::DomainEvent, errors::DomainError};

/// 事件发布器接口
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), DomainError>;
    async fn publish_batch(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), DomainError>;
}

/// 事件处理器接口
#[async_trait]
pub trait EventHandler<T: DomainEvent>: Send + Sync {
    async fn handle(&self, event: &T) -> Result<(), DomainError>;
}

/// 内存事件发布器（用于开发和测试）
#[derive(Debug)]
pub struct InMemoryEventPublisher {
    events: Arc<Mutex<Vec<Box<dyn DomainEvent>>>>,
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<dyn DomainEvent>>>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 获取已发布的事件（用于测试）
    pub async fn get_published_events(&self) -> Vec<String> {
        let events = self.events.lock().await;
        events.iter().map(|e| e.event_name().to_string()).collect()
    }

    /// 清除所有事件（用于测试）
    pub async fn clear_events(&self) {
        self.events.lock().await.clear();
    }

    /// 获取事件数量
    pub async fn event_count(&self) -> usize {
        self.events.lock().await.len()
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), DomainError> {
        log::info!(
            "📢 发布领域事件: {} - 聚合ID: {} - 时间: {}",
            event.event_name(),
            event.aggregate_id(),
            event.occurred_at()
        );

        // 存储事件
        self.events.lock().await.push(event);

        // 这里可以添加事件处理逻辑
        // 在实际应用中，这里会调用注册的事件处理器

        Ok(())
    }

    async fn publish_batch(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), DomainError> {
        let mut event_store = self.events.lock().await;
        
        for event in events {
            log::info!(
                "📢 批量发布领域事件: {} - 聚合ID: {}",
                event.event_name(),
                event.aggregate_id()
            );
            event_store.push(event);
        }

        Ok(())
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shared::events::BaseDomainEvent;

    #[tokio::test]
    async fn should_publish_and_store_event() {
        let publisher = InMemoryEventPublisher::new();
        
        let event = BaseDomainEvent::new(
            "TestEvent".to_string(),
            "test-aggregate-123".to_string(),
            serde_json::json!({"test": "data"}),
        );

        publisher.publish(Box::new(event)).await.unwrap();

        assert_eq!(publisher.event_count().await, 1);
        let published = publisher.get_published_events().await;
        assert_eq!(published[0], "TestEvent");
    }

    #[tokio::test]
    async fn should_publish_batch_events() {
        let publisher = InMemoryEventPublisher::new();
        
        let events = vec![
            Box::new(BaseDomainEvent::new(
                "Event1".to_string(),
                "aggregate-1".to_string(),
                serde_json::json!({}),
            )) as Box<dyn DomainEvent>,
            Box::new(BaseDomainEvent::new(
                "Event2".to_string(),
                "aggregate-2".to_string(),
                serde_json::json!({}),
            )) as Box<dyn DomainEvent>,
        ];

        publisher.publish_batch(events).await.unwrap();

        assert_eq!(publisher.event_count().await, 2);
    }

    #[tokio::test]
    async fn should_clear_events() {
        let publisher = InMemoryEventPublisher::new();
        
        let event = BaseDomainEvent::new(
            "TestEvent".to_string(),
            "test-aggregate".to_string(),
            serde_json::json!({}),
        );

        publisher.publish(Box::new(event)).await.unwrap();
        assert_eq!(publisher.event_count().await, 1);

        publisher.clear_events().await;
        assert_eq!(publisher.event_count().await, 0);
    }
} 