//! # opsforge
//!
//! Rust trait definitions for the eleven Manufacturing Operations Management
//! functions defined by the MESA International standard.
//!
//! Each module corresponds to one MESA function and exposes a single primary
//! trait generic over an identity type `ID`. Implementations (database-backed,
//! in-memory, mock, etc.) live downstream.
//!
//! ## Features
//!
//! | Feature | Enables |
//! |---------|---------|
//! | `default-models` | [`default_models`] — concrete type aliases using [`uuid::Uuid`] |
//! | `serde` | [`serde::Serialize`] / [`serde::Deserialize`] on all opsforge-owned types, and enables `rs95/serde` |
//!
//! ## MESA 11 Functions
//!
//! | # | Module | Trait |
//! |---|--------|-------|
//! | 1 | [`resource_allocation`] | [`ResourceAllocation`] |
//! | 2 | [`scheduling`] | [`DetailedScheduling`] |
//! | 3 | [`dispatching`] | [`Dispatching`] |
//! | 4 | [`document_control`] | [`DocumentControl`] |
//! | 5 | [`data_collection`] | [`DataCollection`] |
//! | 6 | [`labor_management`] | [`LaborManagement`] |
//! | 7 | [`quality_management`] | [`QualityManagement`] |
//! | 8 | [`process_management`] | [`ProcessManagement`] |
//! | 9 | [`maintenance_management`] | [`MaintenanceManagement`] |
//! | 10 | [`product_tracking`] | [`ProductTracking`] |
//! | 11 | [`performance_analysis`] | [`PerformanceAnalysis`] |

#[cfg(feature = "default-models")]
pub mod default_models;

pub mod data_collection;
pub mod dispatching;
pub mod document_control;
pub mod labor_management;
pub mod maintenance_management;
pub mod performance_analysis;
pub mod process_management;
pub mod product_tracking;
pub mod quality_management;
pub mod resource_allocation;
pub mod scheduling;
pub mod types;

// Flat re-exports so users can `use opsforge::ResourceAllocation` etc.
pub use data_collection::DataCollection;
pub use dispatching::Dispatching;
pub use document_control::DocumentControl;
pub use labor_management::LaborManagement;
pub use maintenance_management::MaintenanceManagement;
pub use performance_analysis::PerformanceAnalysis;
pub use process_management::ProcessManagement;
pub use product_tracking::ProductTracking;
pub use quality_management::QualityManagement;
pub use resource_allocation::ResourceAllocation;
pub use scheduling::DetailedScheduling;
