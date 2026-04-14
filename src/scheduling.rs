//! MESA Function 2 — Operations / Detailed Scheduling
//!
//! Sequences manufacturing operations against available resources and
//! capacity, considering priorities, due dates, and material availability.
//! Produces an optimised, time-bounded production schedule.
use crate::types::{OperationId, ResourceId, Result, Timestamp, WorkOrderId};

/// Priority class assigned to a work order for scheduling purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 1,
    Normal = 2,
    High = 3,
    Urgent = 4,
}

/// A single scheduled operation within a work order.
#[derive(Debug, Clone)]
pub struct ScheduledOperation {
    pub operation_id: OperationId,
    pub work_order_id: WorkOrderId,
    pub resource_id: ResourceId,
    pub scheduled_start: Timestamp,
    pub scheduled_end: Timestamp,
    pub priority: Priority,
}

/// A constraint that must be satisfied when scheduling (e.g. a minimum gap
/// between two operations, or a required sequence).
#[derive(Debug, Clone)]
pub struct SchedulingConstraint {
    pub description: String,
}

/// Core interface for detailed operations scheduling (MESA Function 2).
pub trait DetailedScheduling {
    /// Return the current schedule for a work order.
    fn get_schedule(&self, work_order_id: &WorkOrderId) -> Result<Vec<ScheduledOperation>>;

    /// Return all operations currently scheduled on a resource.
    fn schedule_for_resource(&self, resource_id: &ResourceId) -> Result<Vec<ScheduledOperation>>;

    /// Create or replace the schedule for a work order given a set of
    /// constraints. Returns the resulting sequence of scheduled operations.
    fn schedule_work_order(
        &mut self,
        work_order_id: &WorkOrderId,
        priority: Priority,
        constraints: &[SchedulingConstraint],
    ) -> Result<Vec<ScheduledOperation>>;

    /// Move a scheduled operation to a new start time, propagating any
    /// downstream dependencies.
    fn reschedule_operation(
        &mut self,
        operation_id: &OperationId,
        new_start: Timestamp,
    ) -> Result<ScheduledOperation>;

    /// Remove all scheduled operations belonging to a work order.
    fn unschedule_work_order(&mut self, work_order_id: &WorkOrderId) -> Result<()>;
}
