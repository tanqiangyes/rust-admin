use super::DomainError;

/// 验证错误构建器
pub struct ValidationErrorBuilder {
    errors: Vec<String>,
}

impl ValidationErrorBuilder {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, message: impl Into<String>) -> &mut Self {
        self.errors.push(message.into());
        self
    }

    pub fn add_error_if(&mut self, condition: bool, message: impl Into<String>) -> &mut Self {
        if condition {
            self.errors.push(message.into());
        }
        self
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn build(self) -> Result<(), DomainError> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::Validation(self.errors.join("; ")))
        }
    }
}

impl Default for ValidationErrorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 验证宏
#[macro_export]
macro_rules! validate {
    ($validator:expr) => {{
        let mut builder = $crate::domain::shared::errors::validation_error::ValidationErrorBuilder::new();
        $validator(&mut builder);
        builder.build()
    }};
} 