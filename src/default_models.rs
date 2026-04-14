//! Concrete type aliases for all opsforge types, using [`uuid::Uuid`] as the
//! identity type.
//!
//! Most users should import from this module rather than using the generic
//! forms directly. The corresponding ISA-95 types (e.g. `JobOrder`, `MaterialLot`)
//! are available via `rs95::default_models`.
//!
//! # Example
//!
//! ```rust
//! use opsforge::default_models::Allocation;
//! use rs95::default_models::operations::JobOrder;
//! ```
//!
//! Enable with the `default-models` feature flag:
//!
//! ```toml
//! [dependencies]
//! opsforge = { version = "0.1.0", features = ["default-models"] }
//! ```

use uuid::Uuid;

/// Concrete [`Allocation`](crate::resource_allocation::Allocation) using `Uuid` IDs.
pub type Allocation = crate::resource_allocation::Allocation<Uuid>;

/// Concrete [`DataPoint`](crate::data_collection::DataPoint) using `Uuid` IDs.
pub type DataPoint = crate::data_collection::DataPoint<Uuid>;

/// Concrete [`AttendanceRecord`](crate::labor_management::AttendanceRecord) using `Uuid` IDs.
pub type AttendanceRecord = crate::labor_management::AttendanceRecord<Uuid>;

/// Concrete [`NonConformanceReport`](crate::quality_management::NonConformanceReport) using `Uuid` IDs.
pub type NonConformanceReport = crate::quality_management::NonConformanceReport<Uuid>;

/// Concrete [`ProcessAlarm`](crate::process_management::ProcessAlarm) using `Uuid` IDs.
pub type ProcessAlarm = crate::process_management::ProcessAlarm<Uuid>;

/// Concrete [`CorrectiveAction`](crate::process_management::CorrectiveAction) using `Uuid` IDs.
pub type CorrectiveAction = crate::process_management::CorrectiveAction<Uuid>;

/// Concrete [`KpiTarget`](crate::performance_analysis::KpiTarget) using `Uuid` IDs.
pub type KpiTarget = crate::performance_analysis::KpiTarget<Uuid>;

/// Concrete [`OeeReport`](crate::performance_analysis::OeeReport) using `Uuid` IDs.
pub type OeeReport = crate::performance_analysis::OeeReport<Uuid>;
