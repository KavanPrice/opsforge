//! MESA Function 7 — Quality Management
//!
//! Analyses measurements and process data to ensure manufactured products
//! meet specifications. Covers in-process inspections, SPC (statistical
//! process control), non-conformance reporting, and corrective action
//! initiation.
use crate::types::{
    LotId, Measurement, OperationId, QualityEventId, Result, Timestamp, WorkOrderId,
};

/// Disposition decision for a non-conforming item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    Accept,
    ConditionalAccept,
    Rework,
    Scrap,
    ReturnToSupplier,
}

/// Result of a single quality inspection.
#[derive(Debug, Clone)]
pub struct InspectionResult {
    pub event_id: QualityEventId,
    pub work_order_id: WorkOrderId,
    pub lot_id: Option<LotId>,
    pub operation_id: OperationId,
    pub characteristic: String,
    pub measured: Measurement,
    pub lower_spec_limit: Option<f64>,
    pub upper_spec_limit: Option<f64>,
    pub conforming: bool,
    pub inspected_at: Timestamp,
    pub inspector_id: Option<String>,
}

/// A non-conformance report raised against a lot or work order.
#[derive(Debug, Clone)]
pub struct NonConformanceReport {
    pub event_id: QualityEventId,
    pub work_order_id: WorkOrderId,
    pub lot_id: Option<LotId>,
    pub description: String,
    pub disposition: Option<Disposition>,
    pub raised_at: Timestamp,
    pub closed_at: Option<Timestamp>,
}

/// Core interface for quality management (MESA Function 7).
pub trait QualityManagement {
    /// Record an inspection result for an operation.
    fn record_inspection(&mut self, result: InspectionResult) -> Result<InspectionResult>;

    /// Return all inspection results for a work order.
    fn inspections_for_work_order(
        &self,
        work_order_id: &WorkOrderId,
    ) -> Result<Vec<InspectionResult>>;

    /// Return all inspection results for a lot.
    fn inspections_for_lot(&self, lot_id: &LotId) -> Result<Vec<InspectionResult>>;

    /// Raise a non-conformance report.
    fn raise_ncr(
        &mut self,
        work_order_id: &WorkOrderId,
        lot_id: Option<&LotId>,
        description: &str,
        at: Timestamp,
    ) -> Result<NonConformanceReport>;

    /// Apply a disposition decision to an open NCR and close it.
    fn close_ncr(
        &mut self,
        event_id: &QualityEventId,
        disposition: Disposition,
        at: Timestamp,
    ) -> Result<NonConformanceReport>;

    /// Return all open (undisposed) non-conformance reports.
    fn open_ncrs(&self) -> Result<Vec<NonConformanceReport>>;
}
