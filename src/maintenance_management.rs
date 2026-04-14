//! MESA Function 9 — Maintenance Management
//!
//! Tracks planned and unplanned maintenance activities to maximise
//! equipment availability. Covers preventive schedules, corrective work
//! orders, and spare-parts requests. Feeds availability data back to
//! resource allocation and scheduling.
use crate::types::{EquipmentId, MaintenanceTaskId, Result, Timestamp};

/// Distinguishes scheduled preventive maintenance from reactive repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceType {
    Preventive,
    Corrective,
    Predictive,
    ConditionBased,
}

/// Lifecycle state of a maintenance task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceStatus {
    Planned,
    InProgress,
    Completed,
    Deferred,
    Cancelled,
}

/// A maintenance task or work order.
#[derive(Debug, Clone)]
pub struct MaintenanceTask {
    pub id: MaintenanceTaskId,
    pub equipment_id: EquipmentId,
    pub maintenance_type: MaintenanceType,
    pub description: String,
    pub status: MaintenanceStatus,
    pub scheduled_at: Option<Timestamp>,
    pub started_at: Option<Timestamp>,
    pub completed_at: Option<Timestamp>,
    pub technician: Option<String>,
    pub downtime_minutes: Option<u32>,
}

/// Core interface for maintenance management (MESA Function 9).
pub trait MaintenanceManagement {
    /// Create a new planned maintenance task.
    fn plan_task(
        &mut self,
        equipment_id: &EquipmentId,
        maintenance_type: MaintenanceType,
        description: &str,
        scheduled_at: Timestamp,
    ) -> Result<MaintenanceTask>;

    /// Raise an unplanned corrective task (e.g. in response to a breakdown).
    fn raise_corrective(
        &mut self,
        equipment_id: &EquipmentId,
        description: &str,
        at: Timestamp,
    ) -> Result<MaintenanceTask>;

    /// Record that a technician has started work on a task.
    fn start_task(
        &mut self,
        id: &MaintenanceTaskId,
        technician: &str,
        at: Timestamp,
    ) -> Result<MaintenanceTask>;

    /// Complete a task and record the total downtime incurred.
    fn complete_task(
        &mut self,
        id: &MaintenanceTaskId,
        downtime_minutes: u32,
        at: Timestamp,
    ) -> Result<MaintenanceTask>;

    /// Defer a planned task to a new scheduled time.
    fn defer_task(
        &mut self,
        id: &MaintenanceTaskId,
        new_scheduled_at: Timestamp,
    ) -> Result<MaintenanceTask>;

    /// Return all tasks for an equipment item, ordered by scheduled date.
    fn tasks_for_equipment(&self, equipment_id: &EquipmentId) -> Result<Vec<MaintenanceTask>>;

    /// Return all currently open (planned or in-progress) tasks.
    fn open_tasks(&self) -> Result<Vec<MaintenanceTask>>;
}
