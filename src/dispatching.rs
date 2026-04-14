//! MESA Function 3 — Dispatching Production Units
//!
//! Controls the flow of production units (work orders, batches, lots) to
//! the shop floor. Issues work, tracks progress, and manages the
//! prioritised queue of work waiting to be executed.
use crate::types::{LotId, OperationId, ResourceId, Result, Status, Timestamp, WorkOrderId};

/// A work order as it appears on the dispatch list.
#[derive(Debug, Clone)]
pub struct DispatchRecord {
    pub work_order_id: WorkOrderId,
    pub lot_id: Option<LotId>,
    pub operation_id: OperationId,
    pub resource_id: ResourceId,
    pub status: Status,
    pub issued_at: Option<Timestamp>,
    pub started_at: Option<Timestamp>,
    pub completed_at: Option<Timestamp>,
    pub quantity_ordered: f64,
    pub quantity_completed: f64,
}

/// Core interface for dispatching production units (MESA Function 3).
pub trait Dispatching {
    /// Issue a work order to the shop floor, making it visible to operators.
    fn issue(&mut self, work_order_id: &WorkOrderId, at: Timestamp) -> Result<DispatchRecord>;

    /// Record that an operator has started work on an operation.
    fn start_operation(
        &mut self,
        work_order_id: &WorkOrderId,
        operation_id: &OperationId,
        at: Timestamp,
    ) -> Result<DispatchRecord>;

    /// Record completion of an operation and the quantity produced.
    fn complete_operation(
        &mut self,
        work_order_id: &WorkOrderId,
        operation_id: &OperationId,
        quantity_completed: f64,
        at: Timestamp,
    ) -> Result<DispatchRecord>;

    /// Suspend an in-progress work order (e.g. for a quality hold or breakdown).
    fn suspend(
        &mut self,
        work_order_id: &WorkOrderId,
        reason: &str,
        at: Timestamp,
    ) -> Result<DispatchRecord>;

    /// Resume a suspended work order.
    fn resume(&mut self, work_order_id: &WorkOrderId, at: Timestamp) -> Result<DispatchRecord>;

    /// Return the dispatch record for a work order.
    fn get(&self, work_order_id: &WorkOrderId) -> Result<DispatchRecord>;

    /// Return all work orders currently queued or active on a resource.
    fn queue_for_resource(&self, resource_id: &ResourceId) -> Result<Vec<DispatchRecord>>;
}
