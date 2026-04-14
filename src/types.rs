//! Shared primitive types used across all MESA 11 function traits.

/// Opaque identifier for a work order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkOrderId(pub String);

/// Opaque identifier for a production resource (machine, tool, fixture, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(pub String);

/// Opaque identifier for a personnel record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonnelId(pub String);

/// Opaque identifier for a piece of equipment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EquipmentId(pub String);

/// Opaque identifier for a controlled document (recipe, SOP, drawing, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentId(pub String);

/// Opaque identifier for a production lot or batch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LotId(pub String);

/// Opaque identifier for a single manufacturing operation step.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationId(pub String);

/// Opaque identifier for a maintenance task or work order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaintenanceTaskId(pub String);

/// Opaque identifier for a quality event (inspection, NCR, deviation, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualityEventId(pub String);

/// Opaque identifier for a KPI or performance metric definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetricId(pub String);

/// A nanosecond-precision UTC timestamp represented as seconds since the Unix epoch.
/// Implementations may use a richer time library; this type avoids forcing a dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub unix_secs: i64,
    pub nanos: u32,
}

/// High-level lifecycle status shared by several entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    Active,
    Suspended,
    Completed,
    Cancelled,
}

/// A measured quantity with its engineering unit.
#[derive(Debug, Clone)]
pub struct Measurement {
    pub value: f64,
    pub unit: String,
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
