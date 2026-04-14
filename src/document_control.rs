//! MESA Function 4 — Document Control
//!
//! Manages the controlled documents that accompany manufacturing operations:
//! work instructions, recipes, engineering drawings, SOPs, and forms.
//! Ensures operators work from the correct, approved revision.
//!
//! In ISA-95 terms, [`WorkMaster`] represents the master recipe or procedure
//! and [`OperationsDefinition`] provides the detailed operational breakdown.
//! The approval workflow (draft → review → approved → obsolete) is an MES
//! concern not modelled by ISA-95, so [`ApprovalState`] is defined here.

use crate::types::Result;
use rs95::core::operations::{OperationsDefinition, WorkMaster};

/// Revision lifecycle state of a work master or operations definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ApprovalState {
    Draft,
    InReview,
    Approved,
    Obsolete,
}

/// Core interface for document control (MESA Function 4).
pub trait DocumentControl<ID> {
    /// Return a work master (master recipe / procedure) by ID.
    fn get_work_master(&self, id: &ID) -> Result<WorkMaster<ID>>;

    /// Return all work masters, optionally filtered by approval state.
    fn list_work_masters(
        &self,
        state: Option<&ApprovalState>,
    ) -> Result<Vec<WorkMaster<ID>>>;

    /// Persist a new or updated work master.
    fn save_work_master(&mut self, master: WorkMaster<ID>) -> Result<WorkMaster<ID>>;

    /// Return an operations definition by ID.
    fn get_operations_definition(&self, id: &ID) -> Result<OperationsDefinition<ID>>;

    /// Return all operations definitions linked to a work master.
    fn definitions_for_work_master(
        &self,
        work_master_id: &ID,
    ) -> Result<Vec<OperationsDefinition<ID>>>;

    /// Persist a new or updated operations definition.
    fn save_operations_definition(
        &mut self,
        definition: OperationsDefinition<ID>,
    ) -> Result<OperationsDefinition<ID>>;

    /// Transition a work master to a new approval state.
    ///
    /// Implementations should enforce valid state transitions
    /// (e.g. Draft → InReview → Approved → Obsolete).
    fn set_approval_state(
        &mut self,
        work_master_id: &ID,
        state: ApprovalState,
        changed_by: &str,
    ) -> Result<ApprovalState>;
}
