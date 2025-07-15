//! 共享内核 - 跨边界上下文的共享概念

pub mod value_objects;
pub mod events;
pub mod errors;
pub mod entities;

pub use value_objects::*;
pub use events::*;
pub use errors::*;
pub use entities::*; 