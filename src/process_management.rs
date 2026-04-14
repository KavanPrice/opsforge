//! MESA Function 8 — Process Management
//!
//! Monitors the execution of manufacturing processes and detects deviations
//! from the expected process path. Directs operators to take corrective
//! action and records process events for traceability.
use crate::types::{EquipmentId, Measurement, OperationId, Result, Timestamp, WorkOrderId};

/// Severity level of a process alarm or deviation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlarmSeverity {
    Info,
    Warning,
    Critical,
}

/// A process alarm raised when a parameter moves outside acceptable limits.
#[derive(Debug, Clone)]
pub struct ProcessAlarm {
    pub alarm_id: String,
    pub work_order_id: Option<WorkOrderId>,
    pub operation_id: Option<OperationId>,
    pub equipment_id: Option<EquipmentId>,
    pub parameter: String,
    pub actual: Measurement,
    pub severity: AlarmSeverity,
    pub message: String,
    pub raised_at: Timestamp,
    pub acknowledged_at: Option<Timestamp>,
    pub resolved_at: Option<Timestamp>,
}

/// A corrective action directive issued in response to a deviation.
#[derive(Debug, Clone)]
pub struct CorrectiveAction {
    pub action_id: String,
    pub alarm_id: String,
    pub instruction: String,
    pub assigned_to: Option<String>,
    pub issued_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

/// Core interface for process management (MESA Function 8).
pub trait ProcessManagement {
    /// Raise a process alarm against an operation or equipment.
    fn raise_alarm(&mut self, alarm: ProcessAlarm) -> Result<ProcessAlarm>;

    /// Acknowledge an alarm (confirm it has been seen by an operator).
    fn acknowledge_alarm(
        &mut self,
        alarm_id: &str,
        acknowledged_by: &str,
        at: Timestamp,
    ) -> Result<ProcessAlarm>;

    /// Resolve an alarm after the underlying issue has been corrected.
    fn resolve_alarm(&mut self, alarm_id: &str, at: Timestamp) -> Result<ProcessAlarm>;

    /// Return all active (unresolved) alarms, ordered by severity descending.
    fn active_alarms(&self) -> Result<Vec<ProcessAlarm>>;

    /// Issue a corrective action directive in response to a raised alarm.
    fn issue_corrective_action(
        &mut self,
        alarm_id: &str,
        instruction: &str,
        assigned_to: Option<&str>,
        at: Timestamp,
    ) -> Result<CorrectiveAction>;

    /// Complete a corrective action.
    fn complete_corrective_action(
        &mut self,
        action_id: &str,
        at: Timestamp,
    ) -> Result<CorrectiveAction>;

    /// Return the process history (alarms + actions) for a work order.
    fn history_for_work_order(
        &self,
        work_order_id: &WorkOrderId,
    ) -> Result<(Vec<ProcessAlarm>, Vec<CorrectiveAction>)>;
}
