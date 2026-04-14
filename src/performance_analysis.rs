//! MESA Function 11 — Performance Analysis
//!
//! Compares actual manufacturing results against planned targets and
//! historical baselines. Produces KPI reports (OEE, yield, cycle time,
//! scrap rate, etc.) and surfaces trends that drive continuous improvement.
use crate::types::{EquipmentId, MetricId, Result, Timestamp, WorkOrderId};

/// A named performance metric definition with its target value and unit.
#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub id: MetricId,
    pub name: String,
    pub description: String,
    pub unit: String,
    pub target: f64,
    pub lower_control_limit: Option<f64>,
    pub upper_control_limit: Option<f64>,
}

/// A measured KPI value captured at a point in time.
#[derive(Debug, Clone)]
pub struct MetricReading {
    pub metric_id: MetricId,
    pub value: f64,
    pub work_order_id: Option<WorkOrderId>,
    pub equipment_id: Option<EquipmentId>,
    pub period_start: Timestamp,
    pub period_end: Timestamp,
}

/// Overall Equipment Effectiveness breakdown.
#[derive(Debug, Clone)]
pub struct OeeReport {
    pub equipment_id: EquipmentId,
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    /// Availability = run time / planned production time.
    pub availability: f64,
    /// Performance = actual output rate / theoretical maximum rate.
    pub performance: f64,
    /// Quality = good units / total units started.
    pub quality: f64,
    /// OEE = Availability × Performance × Quality.
    pub oee: f64,
}

/// Core interface for performance analysis (MESA Function 11).
pub trait PerformanceAnalysis {
    /// Register a new KPI metric definition.
    fn define_metric(&mut self, definition: MetricDefinition) -> Result<MetricDefinition>;

    /// Return a metric definition by ID.
    fn get_metric(&self, id: &MetricId) -> Result<MetricDefinition>;

    /// Record an actual metric reading for a time period.
    fn record_reading(&mut self, reading: MetricReading) -> Result<()>;

    /// Return all readings for a metric within a time window.
    fn readings_for_metric(
        &self,
        metric_id: &MetricId,
        from: Timestamp,
        to: Timestamp,
    ) -> Result<Vec<MetricReading>>;

    /// Compute and return OEE for a piece of equipment over a time window.
    fn compute_oee(
        &self,
        equipment_id: &EquipmentId,
        from: Timestamp,
        to: Timestamp,
    ) -> Result<OeeReport>;

    /// Return a summary of all metric readings for a completed work order.
    fn work_order_summary(&self, work_order_id: &WorkOrderId) -> Result<Vec<MetricReading>>;
}
