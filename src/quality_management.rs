//! MESA Function 7 — Quality Management
//!
//! Analyses measurements and process data to ensure manufactured products
//! meet specifications. Covers in-process inspections, SPC (statistical
//! process control), non-conformance reporting, and corrective action
//! initiation.
//!
//! ISA-95 models material quality testing via [`MaterialTestSpecification`]
//! and [`QATestResult`], and equipment capability testing via
//! [`EquipmentCapabilityTestSpecification`] and [`EquipmentCapabilityTestResult`].
//! Non-conformance reporting and disposition decisions are MES concerns not
//! covered by ISA-95, so [`NonConformanceReport`] and [`Disposition`] are
//! defined here.

use crate::types::Result;
use rs95::core::{
    equipment::{EquipmentCapabilityTestResult, EquipmentCapabilityTestSpecification},
    material::{MaterialTestSpecification, QATestResult},
};

/// Disposition decision applied to a non-conforming item.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Disposition {
    Accept,
    ConditionalAccept,
    Rework,
    Scrap,
    ReturnToSupplier,
}

/// A non-conformance report raised when a lot or job fails to meet specification.
///
/// The `id` field uses the same `ID` type parameter as the surrounding ISA-95
/// objects so that implementors can use a consistent identity strategy.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct NonConformanceReport<ID> {
    pub id: ID,
    pub job_order_id: ID,
    pub material_lot_id: Option<ID>,
    pub description: String,
    pub disposition: Option<Disposition>,
    /// ISO 8601 timestamp or site-specific format.
    pub raised_at: Option<String>,
    pub closed_at: Option<String>,
}

/// Core interface for quality management (MESA Function 7).
pub trait QualityManagement<ID> {
    /// Return a material test specification by ID.
    fn get_material_test_spec(&self, id: &ID) -> Result<MaterialTestSpecification<ID>>;

    /// Record a material quality test result.
    fn record_material_test(
        &mut self,
        result: QATestResult<ID>,
    ) -> Result<QATestResult<ID>>;

    /// Return all material test results for a given material lot.
    fn results_for_lot(&self, lot_id: &ID) -> Result<Vec<QATestResult<ID>>>;

    /// Return an equipment capability test specification by ID.
    fn get_equipment_capability_spec(
        &self,
        id: &ID,
    ) -> Result<EquipmentCapabilityTestSpecification<ID>>;

    /// Record an equipment capability test result.
    fn record_equipment_capability(
        &mut self,
        result: EquipmentCapabilityTestResult<ID>,
    ) -> Result<EquipmentCapabilityTestResult<ID>>;

    /// Raise a non-conformance report against a job order or lot.
    fn raise_ncr(
        &mut self,
        ncr: NonConformanceReport<ID>,
    ) -> Result<NonConformanceReport<ID>>;

    /// Apply a disposition decision to an open NCR and close it.
    fn close_ncr(
        &mut self,
        ncr_id: &ID,
        disposition: Disposition,
        closed_at: Option<String>,
    ) -> Result<NonConformanceReport<ID>>;

    /// Return all open (undisposed) non-conformance reports.
    fn open_ncrs(&self) -> Result<Vec<NonConformanceReport<ID>>>;
}
