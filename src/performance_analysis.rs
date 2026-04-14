//! MESA Function 11 — Performance Analysis
//!
//! Compares actual manufacturing results against planned targets and
//! historical baselines. Produces KPI reports (OEE, yield, cycle time,
//! scrap rate, etc.) and surfaces trends that drive continuous improvement.
//!
//! ISA-95 captures actual performance data in [`OperationsPerformance`] and
//! declared capability in [`OperationsCapability`]. KPI target definitions
//! and OEE calculations are MES analytical concerns not modelled by ISA-95,
//! so [`KpiTarget`] and [`OeeReport`] are defined here.

use crate::types::Result;
use rs95::core::operations::{OperationsCapability, OperationsPerformance};

/// A named KPI with its target value and statistical control limits.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct KpiTarget<ID> {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub unit: String,
    pub target: f64,
    pub lower_control_limit: Option<f64>,
    pub upper_control_limit: Option<f64>,
}

/// Overall Equipment Effectiveness breakdown for a piece of equipment over a
/// given time period.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct OeeReport<ID> {
    pub equipment_id: ID,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    /// Availability = run time / planned production time.
    pub availability: f64,
    /// Performance = actual output rate / theoretical maximum rate.
    pub performance: f64,
    /// Quality = good units / total units started.
    pub quality: f64,
    /// OEE = Availability × Performance × Quality.
    pub oee: f64,
}

/// Core interface for performance analysis (MESA Function 11).
pub trait PerformanceAnalysis<ID> {
    /// Persist a KPI target definition.
    fn define_kpi(&mut self, target: KpiTarget<ID>) -> Result<KpiTarget<ID>>;

    /// Return a KPI target definition by ID.
    fn get_kpi(&self, id: &ID) -> Result<KpiTarget<ID>>;

    /// Record actual operations performance data for a time period.
    fn record_performance(
        &mut self,
        performance: OperationsPerformance<ID>,
    ) -> Result<OperationsPerformance<ID>>;

    /// Return operations performance data by ID.
    fn get_performance(&self, id: &ID) -> Result<OperationsPerformance<ID>>;

    /// Return all performance records within an optional time window
    /// (`from` and `to` as ISO 8601 strings).
    fn performance_in_period(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<OperationsPerformance<ID>>>;

    /// Return the operations capability declaration for a resource.
    fn get_capability(&self, id: &ID) -> Result<OperationsCapability<ID>>;

    /// Compute OEE for a piece of equipment over a time window.
    fn compute_oee(
        &self,
        equipment_id: &ID,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<OeeReport<ID>>;
}
