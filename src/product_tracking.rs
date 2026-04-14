//! MESA Function 10 — Product Tracking & Genealogy
//!
//! Provides end-to-end visibility into where production units are in the
//! process and a historical record of every operation, component, and
//! measurement associated with a finished product. Supports forward and
//! reverse traceability from raw material through to shipment.
use crate::types::{LotId, OperationId, ResourceId, Result, Timestamp, WorkOrderId};

/// Current location and status of a tracked lot in the production process.
#[derive(Debug, Clone)]
pub struct LotStatus {
    pub lot_id: LotId,
    pub work_order_id: WorkOrderId,
    pub current_operation_id: Option<OperationId>,
    pub current_resource_id: Option<ResourceId>,
    pub quantity_remaining: f64,
    pub last_updated: Timestamp,
}

/// A single traceability event recorded against a lot.
#[derive(Debug, Clone)]
pub struct TraceabilityEvent {
    pub lot_id: LotId,
    pub work_order_id: WorkOrderId,
    pub operation_id: OperationId,
    pub event_type: String,
    pub detail: String,
    pub recorded_at: Timestamp,
}

/// A parent–child component relationship used to build genealogy trees.
#[derive(Debug, Clone)]
pub struct GenealogyLink {
    pub parent_lot_id: LotId,
    pub child_lot_id: LotId,
    pub quantity_consumed: f64,
    pub linked_at: Timestamp,
}

/// Core interface for product tracking and genealogy (MESA Function 10).
pub trait ProductTracking {
    /// Return the current status and location of a lot.
    fn status(&self, lot_id: &LotId) -> Result<LotStatus>;

    /// Return all lots currently active within a work order.
    fn lots_for_work_order(&self, work_order_id: &WorkOrderId) -> Result<Vec<LotStatus>>;

    /// Record a traceability event (operation start/end, measurement, hold, etc.).
    fn record_event(&mut self, event: TraceabilityEvent) -> Result<()>;

    /// Return the full ordered history of events for a lot.
    fn history(&self, lot_id: &LotId) -> Result<Vec<TraceabilityEvent>>;

    /// Record that a child lot was consumed into a parent (component linkage).
    fn link_component(
        &mut self,
        parent_lot_id: &LotId,
        child_lot_id: &LotId,
        quantity_consumed: f64,
        at: Timestamp,
    ) -> Result<GenealogyLink>;

    /// Return all direct children (components) of a parent lot.
    fn children(&self, parent_lot_id: &LotId) -> Result<Vec<GenealogyLink>>;

    /// Return all parent lots that consumed a given child lot.
    fn parents(&self, child_lot_id: &LotId) -> Result<Vec<GenealogyLink>>;
}
