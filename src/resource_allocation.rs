//! MESA Function 1 — Resource Allocation & Status
//!
//! Manages the reservation and real-time status of all resources required
//! to execute manufacturing operations: machines, tools, fixtures, labour,
//! and materials. Ensures availability information is visible to scheduling
//! and dispatching functions.

use crate::types::Result;
use rs95::core::{
    equipment::Equipment,
    equipment_hierarchy::{WorkCenter, WorkUnit},
    material::MaterialLot,
    operations::OperationsCapability,
    personnel::Person,
    physical_asset::PhysicalAsset,
};

/// A time-bounded reservation of a resource against a job order.
///
/// Times are `Option<String>` to accept any ISO 8601 or site-specific format,
/// consistent with the rs95 convention.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct Allocation<ID> {
    pub resource_id: ID,
    pub job_order_id: ID,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Core interface for resource allocation and status management (MESA Function 1).
pub trait ResourceAllocation<ID> {
    /// Return an equipment resource by ID.
    fn get_equipment(&self, id: &ID) -> Result<Equipment<ID>>;

    /// Return all work centres, optionally filtered by equipment class ID.
    fn list_work_centers(
        &self,
        equipment_class_id: Option<&ID>,
    ) -> Result<Vec<WorkCenter<ID>>>;

    /// Return all work units within a work centre.
    fn list_work_units(&self, work_center_id: &ID) -> Result<Vec<WorkUnit<ID>>>;

    /// Return the current operations capability declaration for a resource.
    fn get_capability(&self, id: &ID) -> Result<OperationsCapability<ID>>;

    /// Publish an updated operations capability for a resource.
    fn update_capability(
        &mut self,
        capability: OperationsCapability<ID>,
    ) -> Result<OperationsCapability<ID>>;

    /// Reserve a resource for a job order over the given time window.
    fn allocate(
        &mut self,
        resource_id: ID,
        job_order_id: ID,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Result<Allocation<ID>>;

    /// Release a previously created allocation.
    fn release(&mut self, resource_id: &ID, job_order_id: &ID) -> Result<()>;

    /// Return the current allocations for a resource.
    fn current_allocations(&self, resource_id: &ID) -> Result<Vec<Allocation<ID>>>;

    /// Return a physical asset (tool, fixture, etc.) by ID.
    fn get_physical_asset(&self, id: &ID) -> Result<PhysicalAsset<ID>>;

    /// Return a person (labour resource) by ID.
    fn get_person(&self, id: &ID) -> Result<Person<ID>>;

    /// Return a material lot (consumable resource) by ID.
    fn get_material_lot(&self, id: &ID) -> Result<MaterialLot<ID>>;
}
