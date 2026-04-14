//! MESA Function 6 — Labor Management
//!
//! Tracks personnel attendance, shift assignments, certifications, and
//! labour utilisation. Provides visibility into workforce status and
//! ensures only qualified personnel are assigned to operations that
//! require specific skills or certifications.
use crate::types::{OperationId, PersonnelId, Result, Timestamp};

/// Current attendance / shift status of a worker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttendanceStatus {
    ClockedIn,
    OnBreak,
    ClockedOut,
    Absent,
}

/// A certification or qualification held by a worker.
#[derive(Debug, Clone)]
pub struct Certification {
    pub name: String,
    pub issued_at: Timestamp,
    pub expires_at: Option<Timestamp>,
}

/// A personnel record including qualifications and current status.
#[derive(Debug, Clone)]
pub struct PersonnelRecord {
    pub id: PersonnelId,
    pub name: String,
    pub department: String,
    pub attendance: AttendanceStatus,
    pub certifications: Vec<Certification>,
}

/// An assignment of a worker to a specific operation.
#[derive(Debug, Clone)]
pub struct LaborAssignment {
    pub personnel_id: PersonnelId,
    pub operation_id: OperationId,
    pub start: Timestamp,
    pub end: Option<Timestamp>,
    pub role: String,
}

/// Core interface for labor management (MESA Function 6).
pub trait LaborManagement {
    /// Return the personnel record for a worker.
    fn get_personnel(&self, id: &PersonnelId) -> Result<PersonnelRecord>;

    /// Return all workers currently clocked in.
    fn active_workforce(&self) -> Result<Vec<PersonnelRecord>>;

    /// Clock a worker in at the given time.
    fn clock_in(&mut self, id: &PersonnelId, at: Timestamp) -> Result<PersonnelRecord>;

    /// Clock a worker out at the given time.
    fn clock_out(&mut self, id: &PersonnelId, at: Timestamp) -> Result<PersonnelRecord>;

    /// Assign a worker to an operation with a specific role.
    fn assign(
        &mut self,
        personnel_id: &PersonnelId,
        operation_id: &OperationId,
        role: &str,
        at: Timestamp,
    ) -> Result<LaborAssignment>;

    /// Complete a labor assignment (worker finished the operation).
    fn complete_assignment(
        &mut self,
        personnel_id: &PersonnelId,
        operation_id: &OperationId,
        at: Timestamp,
    ) -> Result<LaborAssignment>;

    /// Return all workers qualified for a given certification name.
    fn qualified_for(&self, certification: &str) -> Result<Vec<PersonnelRecord>>;
}
