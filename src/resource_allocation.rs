//! MESA Function 1 — Resource Allocation & Status
//!
//! Manages the reservation and real-time status of all resources required
//! to execute manufacturing operations: machines, tools, fixtures, labour,
//! and materials. Ensures availability information is visible to scheduling
//! and dispatching functions.
use crate::types::{OperationId, ResourceId, Result, Status, Timestamp};

/// The capability, physical location, and current availability of a resource.
#[derive(Debug, Clone)]
pub struct ResourceRecord {
    pub id: ResourceId,
    pub name: String,
    pub status: Status,
    pub capabilities: Vec<String>,
    pub last_updated: Timestamp,
}

/// A time-bounded reservation of a resource against an operation.
#[derive(Debug, Clone)]
pub struct Allocation {
    pub resource_id: ResourceId,
    pub operation_id: OperationId,
    pub start: Timestamp,
    pub end: Timestamp,
}

/// Core interface for resource allocation and status management (MESA Function 1).
pub trait ResourceAllocation {
    /// Return the current record for a single resource.
    fn get_resource(&self, id: &ResourceId) -> Result<ResourceRecord>;

    /// Return all resources, optionally filtered to those matching `capability`.
    fn list_resources(&self, capability: Option<&str>) -> Result<Vec<ResourceRecord>>;

    /// Reserve a resource for an operation over the given time window.
    /// Returns the created [`Allocation`].
    fn allocate(
        &mut self,
        resource_id: &ResourceId,
        operation_id: &OperationId,
        start: Timestamp,
        end: Timestamp,
    ) -> Result<Allocation>;

    /// Release a previously created allocation before its scheduled end.
    fn release(&mut self, resource_id: &ResourceId, operation_id: &OperationId) -> Result<()>;

    /// Update the operational status of a resource (e.g. Active → Suspended).
    fn set_status(&mut self, id: &ResourceId, status: Status) -> Result<()>;

    /// Return all allocations currently active for a resource.
    fn current_allocations(&self, resource_id: &ResourceId) -> Result<Vec<Allocation>>;
}
