//! 时间戳值对象

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// 时间戳值对象，封装时间相关的业务逻辑
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp {
    value: DateTime<Utc>,
}

impl Timestamp {
    /// 创建当前时间戳
    pub fn now() -> Self {
        Self {
            value: Utc::now(),
        }
    }

    /// 从DateTime创建
    pub fn from_datetime(datetime: DateTime<Utc>) -> Self {
        Self { value: datetime }
    }

    /// 从ISO字符串创建
    pub fn from_iso_string(iso_string: &str) -> Result<Self, chrono::ParseError> {
        let datetime = DateTime::parse_from_rfc3339(iso_string)?
            .with_timezone(&Utc);
        Ok(Self { value: datetime })
    }

    /// 获取内部值
    pub fn value(&self) -> DateTime<Utc> {
        self.value
    }

    /// 转换为ISO字符串
    pub fn to_iso_string(&self) -> String {
        self.value.to_rfc3339()
    }

    /// 检查是否在指定时间之前
    pub fn is_before(&self, other: &Timestamp) -> bool {
        self.value < other.value
    }

    /// 检查是否在指定时间之后
    pub fn is_after(&self, other: &Timestamp) -> bool {
        self.value > other.value
    }

    /// 增加秒数
    pub fn add_seconds(&self, seconds: i64) -> Self {
        Self {
            value: self.value + Duration::seconds(seconds),
        }
    }

    /// 增加分钟
    pub fn add_minutes(&self, minutes: i64) -> Self {
        Self {
            value: self.value + Duration::minutes(minutes),
        }
    }

    /// 增加小时
    pub fn add_hours(&self, hours: i64) -> Self {
        Self {
            value: self.value + Duration::hours(hours),
        }
    }

    /// 增加天数
    pub fn add_days(&self, days: i64) -> Self {
        Self {
            value: self.value + Duration::days(days),
        }
    }

    /// 计算与另一个时间戳的差值（秒）
    pub fn seconds_since(&self, other: &Timestamp) -> i64 {
        (self.value - other.value).num_seconds()
    }

    /// 检查是否过期（相对于当前时间）
    pub fn is_expired(&self) -> bool {
        self.value < Utc::now()
    }
}

impl Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timestamp({})", self.value.format("%Y-%m-%d %H:%M:%S UTC"))
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.format("%Y-%m-%d %H:%M:%S UTC"))
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_timestamp_from_now() {
        let ts = Timestamp::now();
        assert!(ts.value() <= Utc::now());
    }

    #[test]
    fn should_add_duration() {
        let ts = Timestamp::now();
        let future = ts.add_hours(1);
        assert!(future.is_after(&ts));
        assert_eq!(future.seconds_since(&ts), 3600);
    }

    #[test]
    fn should_parse_iso_string() {
        let iso = "2023-12-25T10:30:00Z";
        let ts = Timestamp::from_iso_string(iso).unwrap();
        assert_eq!(ts.to_iso_string(), "2023-12-25T10:30:00+00:00");
    }

    #[test]
    fn should_compare_timestamps() {
        let ts1 = Timestamp::now();
        let ts2 = ts1.add_minutes(1);
        
        assert!(ts2.is_after(&ts1));
        assert!(ts1.is_before(&ts2));
    }
} 