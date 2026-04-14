//! MESA Function 8 — Process Management
//!
//! Monitors the execution of manufacturing processes and detects deviations
//! from the expected process path. Directs operators to take corrective
//! action and records process events for traceability.
//!
//! ISA-95 captures structured execution outcomes via [`OperationsResponse`]
//! and aggregated performance data via [`OperationsPerformance`]. Process
//! alarms and corrective actions — which represent real-time deviation
//! management — are MES operational concerns not modelled by ISA-95, so
//! [`ProcessAlarm`], [`AlarmSeverity`], and [`CorrectiveAction`] are defined here.

use crate::types::Result;
use rs95::core::operations::{OperationsPerformance, OperationsResponse};

/// Severity level of a process alarm or deviation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlarmSeverity {
    Info,
    Warning,
    Critical,
}

/// A process alarm raised when a parameter moves outside acceptable limits.
#[derive(Debug, Clone)]
pub struct ProcessAlarm<ID> {
    pub id: ID,
    pub job_order_id: Option<ID>,
    pub equipment_id: Option<ID>,
    /// Name of the process parameter that triggered the alarm.
    pub parameter: String,
    /// Actual value that triggered the alarm, as a string.
    pub actual_value: String,
    pub unit: Option<String>,
    pub severity: AlarmSeverity,
    pub message: String,
    pub raised_at: Option<String>,
    pub acknowledged_at: Option<String>,
    pub resolved_at: Option<String>,
}

/// A corrective action directive issued in response to a process alarm.
#[derive(Debug, Clone)]
pub struct CorrectiveAction<ID> {
    pub id: ID,
    pub alarm_id: ID,
    pub instruction: String,
    pub assigned_to: Option<String>,
    pub issued_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Core interface for process management (MESA Function 8).
pub trait ProcessManagement<ID> {
    /// Raise a process alarm against a job order or piece of equipment.
    fn raise_alarm(&mut self, alarm: ProcessAlarm<ID>) -> Result<ProcessAlarm<ID>>;

    /// Acknowledge an alarm (confirm it has been seen by an operator).
    fn acknowledge_alarm(
        &mut self,
        alarm_id: &ID,
        acknowledged_by: &str,
        at: Option<String>,
    ) -> Result<ProcessAlarm<ID>>;

    /// Resolve an alarm once the underlying deviation has been corrected.
    fn resolve_alarm(
        &mut self,
        alarm_id: &ID,
        at: Option<String>,
    ) -> Result<ProcessAlarm<ID>>;

    /// Return all active (unresolved) alarms, ordered by severity descending.
    fn active_alarms(&self) -> Result<Vec<ProcessAlarm<ID>>>;

    /// Issue a corrective action directive in response to an alarm.
    fn issue_corrective_action(
        &mut self,
        action: CorrectiveAction<ID>,
    ) -> Result<CorrectiveAction<ID>>;

    /// Complete a corrective action.
    fn complete_corrective_action(
        &mut self,
        action_id: &ID,
        at: Option<String>,
    ) -> Result<CorrectiveAction<ID>>;

    /// Record the ISA-95 operations response for a completed or aborted request.
    fn record_operations_response(
        &mut self,
        response: OperationsResponse<ID>,
    ) -> Result<OperationsResponse<ID>>;

    /// Return aggregated operations performance data for a time period.
    fn get_operations_performance(&self, id: &ID) -> Result<OperationsPerformance<ID>>;
}
