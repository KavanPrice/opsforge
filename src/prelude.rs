//! Convenience re-exports for crates implementing the MESA 11 functions.
//!
//! Importing this module covers all eleven traits, the `Result` alias, and
//! every opsforge-owned supporting type. ISA-95 domain objects (e.g.
//! `JobOrder`, `MaterialLot`) should be imported from `rs95` directly.
//!
//! # Example
//!
//! ```rust
//! use opsforge::prelude::*;
//! ```

// Traits
pub use crate::data_collection::DataCollection;
pub use crate::dispatching::Dispatching;
pub use crate::document_control::DocumentControl;
pub use crate::labor_management::LaborManagement;
pub use crate::maintenance_management::MaintenanceManagement;
pub use crate::performance_analysis::PerformanceAnalysis;
pub use crate::process_management::ProcessManagement;
pub use crate::product_tracking::ProductTracking;
pub use crate::quality_management::QualityManagement;
pub use crate::resource_allocation::ResourceAllocation;
pub use crate::scheduling::DetailedScheduling;

// Result alias
pub use crate::types::Result;

// Opsforge-owned supporting types
pub use crate::data_collection::{DataPoint, DataSource};
pub use crate::document_control::ApprovalState;
pub use crate::labor_management::AttendanceRecord;
pub use crate::performance_analysis::{KpiTarget, OeeReport};
pub use crate::process_management::{AlarmSeverity, CorrectiveAction, ProcessAlarm};
pub use crate::quality_management::{Disposition, NonConformanceReport};
pub use crate::resource_allocation::Allocation;
