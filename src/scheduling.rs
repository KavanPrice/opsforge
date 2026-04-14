//! MESA Function 2 — Operations / Detailed Scheduling
//!
//! Sequences manufacturing operations against available resources and
//! capacity, considering priorities, due dates, and material availability.
//! Produces an optimised, time-bounded production schedule.

use crate::types::Result;
use rs95::core::operations::{
    OperationsCapability, OperationsDefinition, OperationsRequest, OperationsSchedule,
};

/// Core interface for detailed operations scheduling (MESA Function 2).
pub trait DetailedScheduling<ID> {
    /// Return a schedule by ID.
    fn get_schedule(&self, id: &ID) -> Result<OperationsSchedule<ID>>;

    /// Publish a new or updated operations schedule.
    fn create_schedule(
        &mut self,
        schedule: OperationsSchedule<ID>,
    ) -> Result<OperationsSchedule<ID>>;

    /// Return an individual operations request by ID.
    fn get_request(&self, id: &ID) -> Result<OperationsRequest<ID>>;

    /// Add or update an operations request within an existing schedule.
    fn update_request(
        &mut self,
        request: OperationsRequest<ID>,
    ) -> Result<OperationsRequest<ID>>;

    /// Remove an operations request from its schedule.
    fn cancel_request(&mut self, id: &ID) -> Result<()>;

    /// Return the operations definition that describes how a given request
    /// should be executed.
    fn get_operations_definition(&self, id: &ID) -> Result<OperationsDefinition<ID>>;

    /// Return the current capability declaration used as the basis for
    /// feasibility checks during scheduling.
    fn get_capability(&self, id: &ID) -> Result<OperationsCapability<ID>>;
}
