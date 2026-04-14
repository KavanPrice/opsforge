//! MESA Function 6 — Labor Management
//!
//! Tracks personnel attendance, shift assignments, certifications, and
//! labour utilisation. Provides visibility into workforce status and
//! ensures only qualified personnel are assigned to operations that
//! require specific skills or certifications.
//!
//! ISA-95 models personnel classifications ([`PersonnelClass`]), individual
//! workers ([`Person`]), and qualification results ([`QualificationTestResult`]).
//! Attendance tracking (clock-in/out) is an MES operational concern not
//! covered by ISA-95, so [`AttendanceRecord`] is defined here.

use crate::types::Result;
use rs95::core::{
    personnel::{Person, PersonnelClass, QualificationTestResult, QualificationTestSpecification},
    process_segment::PersonnelSegmentSpecification,
};

/// A clock-in or clock-out attendance event for a worker.
#[derive(Debug, Clone)]
pub struct AttendanceRecord<ID> {
    pub person_id: ID,
    pub clocked_in_at: Option<String>,
    pub clocked_out_at: Option<String>,
    pub shift: Option<String>,
}

/// Core interface for labor management (MESA Function 6).
pub trait LaborManagement<ID> {
    /// Return a worker by ID.
    fn get_person(&self, id: &ID) -> Result<Person<ID>>;

    /// Return a personnel class (role, skill category) by ID.
    fn get_personnel_class(&self, id: &ID) -> Result<PersonnelClass<ID>>;

    /// Return all workers currently clocked in.
    fn active_workforce(&self) -> Result<Vec<Person<ID>>>;

    /// Record a clock-in or clock-out event for a worker.
    fn record_attendance(
        &mut self,
        record: AttendanceRecord<ID>,
    ) -> Result<AttendanceRecord<ID>>;

    /// Return the most recent attendance record for a worker.
    fn attendance_for_person(&self, person_id: &ID) -> Result<Option<AttendanceRecord<ID>>>;

    /// Return the qualification test specification for a given certification.
    fn get_qualification_spec(
        &self,
        id: &ID,
    ) -> Result<QualificationTestSpecification<ID>>;

    /// Record a qualification test result for a worker.
    fn record_qualification(
        &mut self,
        result: QualificationTestResult<ID>,
    ) -> Result<QualificationTestResult<ID>>;

    /// Return all workers whose qualification results satisfy the given
    /// test specification.
    fn qualified_for_spec(
        &self,
        spec_id: &ID,
    ) -> Result<Vec<Person<ID>>>;

    /// Return the personnel segment specifications (skill/quantity requirements)
    /// for an operations segment.
    fn personnel_requirements(
        &self,
        operations_segment_id: &ID,
    ) -> Result<Vec<PersonnelSegmentSpecification<ID>>>;
}
