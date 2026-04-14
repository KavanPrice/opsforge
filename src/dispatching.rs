//! MESA Function 3 — Dispatching Production Units
//!
//! Controls the flow of production units (work orders, batches, lots) to
//! the shop floor. Issues work, tracks progress, and manages the
//! prioritised queue of work waiting to be executed.

use crate::types::Result;
use rs95::core::operations::{
    JobOrder, JobOrderCommandType, JobOrderStatus, OperationsResponse,
};

/// Core interface for dispatching production units (MESA Function 3).
pub trait Dispatching<ID> {
    /// Issue a job order to the shop floor, making it visible to operators.
    fn issue(&mut self, order: JobOrder<ID>) -> Result<JobOrder<ID>>;

    /// Send a command to an active job order (Start, Stop, Hold, Restart, Abort).
    fn send_command(
        &mut self,
        job_order_id: &ID,
        command: JobOrderCommandType,
    ) -> Result<JobOrder<ID>>;

    /// Return a job order by ID.
    fn get_job_order(&self, id: &ID) -> Result<JobOrder<ID>>;

    /// Return all job orders matching the given status.
    fn list_by_status(&self, status: JobOrderStatus) -> Result<Vec<JobOrder<ID>>>;

    /// Record the operations response for a completed or aborted job order.
    fn record_response(
        &mut self,
        response: OperationsResponse<ID>,
    ) -> Result<OperationsResponse<ID>>;

    /// Return the operations response for a job order.
    fn get_response(&self, job_order_id: &ID) -> Result<OperationsResponse<ID>>;
}
