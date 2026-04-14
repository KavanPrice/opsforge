//! MESA Function 9 — Maintenance Management
//!
//! Tracks planned and unplanned maintenance activities to maximise
//! equipment availability. Covers preventive schedules, corrective work
//! orders, and spare-parts requests. Feeds availability data back to
//! resource allocation and scheduling.
//!
//! ISA-95 represents maintenance work orders as [`JobOrder`] with
//! `work_type = OperationType::Maintenance`. Physical assets subject to
//! maintenance are modelled as [`PhysicalAsset`], and their capability
//! is verified via [`PhysicalAssetCapabilityTestResult`].

use crate::types::Result;
use rs95::core::{
    operations::{JobOrder, JobResponse, OperationType},
    physical_asset::{
        PhysicalAsset, PhysicalAssetCapabilityTestResult, PhysicalAssetCapabilityTestSpecification,
    },
};

/// Core interface for maintenance management (MESA Function 9).
pub trait MaintenanceManagement<ID> {
    /// Return a physical asset by ID.
    fn get_physical_asset(&self, id: &ID) -> Result<PhysicalAsset<ID>>;

    /// Create a maintenance job order.
    ///
    /// Implementations should enforce `order.work_type == OperationType::Maintenance`.
    fn create_maintenance_order(&mut self, order: JobOrder<ID>) -> Result<JobOrder<ID>>;

    /// Return all maintenance job orders matching the given operation type.
    ///
    /// Pass `OperationType::Maintenance` for standard maintenance orders.
    fn list_maintenance_orders(
        &self,
        work_type: OperationType,
    ) -> Result<Vec<JobOrder<ID>>>;

    /// Record the job response (outcome) for a completed maintenance order.
    fn record_job_response(&mut self, response: JobResponse<ID>) -> Result<JobResponse<ID>>;

    /// Return the capability test specification for a physical asset class.
    fn get_capability_spec(
        &self,
        id: &ID,
    ) -> Result<PhysicalAssetCapabilityTestSpecification<ID>>;

    /// Record the result of a physical asset capability test (e.g. post-maintenance
    /// inspection confirming the asset is fit for service).
    fn record_capability_test(
        &mut self,
        result: PhysicalAssetCapabilityTestResult<ID>,
    ) -> Result<PhysicalAssetCapabilityTestResult<ID>>;

    /// Return all capability test results for a physical asset.
    fn capability_history(
        &self,
        asset_id: &ID,
    ) -> Result<Vec<PhysicalAssetCapabilityTestResult<ID>>>;
}
