//! MESA Function 5 — Data Collection / Acquisition
//!
//! Acquires and presents operational data generated during manufacturing:
//! machine signals, operator entries, sensor readings, SCADA feeds, and
//! barcode scans. Acts as the integration boundary between plant-floor
//! systems and higher-level MOM functions.
use crate::types::{EquipmentId, Measurement, OperationId, Result, Timestamp, WorkOrderId};

/// The source type of a collected data point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    ManualEntry,
    Scada,
    Plc,
    Sensor,
    Barcode,
    Rfid,
    Other(String),
}

/// A single captured data event.
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub tag: String,
    pub value: Measurement,
    pub source: DataSource,
    pub equipment_id: Option<EquipmentId>,
    pub operation_id: Option<OperationId>,
    pub work_order_id: Option<WorkOrderId>,
    pub captured_at: Timestamp,
}

/// Core interface for data collection and acquisition (MESA Function 5).
pub trait DataCollection {
    /// Record a single data point against an operation or work order.
    fn record(&mut self, point: DataPoint) -> Result<()>;

    /// Record multiple data points in a single call (batch ingestion).
    fn record_batch(&mut self, points: Vec<DataPoint>) -> Result<()>;

    /// Return all data points captured for an operation.
    fn query_by_operation(&self, operation_id: &OperationId) -> Result<Vec<DataPoint>>;

    /// Return all data points captured for a work order.
    fn query_by_work_order(&self, work_order_id: &WorkOrderId) -> Result<Vec<DataPoint>>;

    /// Return all data points captured from an equipment source within a time window.
    fn query_by_equipment(
        &self,
        equipment_id: &EquipmentId,
        from: Timestamp,
        to: Timestamp,
    ) -> Result<Vec<DataPoint>>;

    /// Return the most recent value for a named tag.
    fn latest(&self, tag: &str) -> Result<Option<DataPoint>>;
}
