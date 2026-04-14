//! Shared primitive types used across all MESA 11 function traits.
//!
//! The domain objects (equipment, material, personnel, operations, etc.) are
//! provided by the [`rs95`] crate. Only types that MESA defines but ISA-95
//! does not model are defined here.

/// Convenience `Result` alias used throughout the crate.
///
/// The error type is a boxed, thread-safe trait object so that implementors
/// can surface any error without this crate imposing a concrete error type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
