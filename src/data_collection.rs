//! MESA Function 5 — Data Collection / Acquisition
//!
//! Acquires and presents operational data generated during manufacturing:
//! machine signals, operator entries, sensor readings, SCADA feeds, and
//! barcode scans. Acts as the integration boundary between plant-floor
//! systems and higher-level MOM functions.
//!
//! Structured execution outcomes (what equipment, material, and personnel
//! were actually used) are captured via ISA-95 [`SegmentResponse`] and
//! [`JobResponse`]. Raw time-series signal capture — which ISA-95 does not
//! model — is handled by [`DataPoint`] defined in this module.

use crate::types::Result;
use rs95::core::operations::{JobResponse, SegmentResponse};

/// The integration source of a raw data point.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataSource {
    ManualEntry,
    Scada,
    Plc,
    Sensor,
    Barcode,
    Rfid,
    Other(String),
}

/// A single raw signal or operator-entered value captured from the plant floor.
///
/// For structured execution results (actual resource consumption, segment
/// outcomes) use [`SegmentResponse`] or [`JobResponse`] instead.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "ID: serde::Serialize",
        deserialize = "ID: serde::Deserialize<'de>"
    ))
)]
pub struct DataPoint<ID> {
    /// Tag name or signal identifier (e.g. `"reactor1.temperature"`).
    pub tag: String,
    /// Raw value as a string to avoid imposing a numeric representation.
    pub value: String,
    /// Engineering unit (e.g. `"degC"`, `"bar"`, `"kg"`).
    pub unit: Option<String>,
    pub source: DataSource,
    pub equipment_id: Option<ID>,
    pub job_order_id: Option<ID>,
    /// Timestamp as an ISO 8601 string or site-specific format.
    pub captured_at: Option<String>,
}

/// Core interface for data collection and acquisition (MESA Function 5).
pub trait DataCollection<ID> {
    /// Record a single raw data point.
    fn record(&mut self, point: DataPoint<ID>) -> Result<()>;

    /// Record multiple raw data points in a single call (batch ingestion).
    fn record_batch(&mut self, points: Vec<DataPoint<ID>>) -> Result<()>;

    /// Return the most recent value for a named tag.
    fn latest(&self, tag: &str) -> Result<Option<DataPoint<ID>>>;

    /// Return all raw data points captured from a piece of equipment within
    /// an optional time window (`from` and `to` as ISO 8601 strings).
    fn query_by_equipment(
        &self,
        equipment_id: &ID,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<DataPoint<ID>>>;

    /// Record the structured outcome of a completed operations segment.
    fn record_segment_response(&mut self, response: SegmentResponse<ID>) -> Result<()>;

    /// Record the structured outcome of a completed job.
    fn record_job_response(&mut self, response: JobResponse<ID>) -> Result<()>;

    /// Return all segment responses associated with a job order.
    fn segment_responses_for_job(
        &self,
        job_order_id: &ID,
    ) -> Result<Vec<SegmentResponse<ID>>>;

    /// Return all job responses associated with a job order.
    fn job_responses_for_job(
        &self,
        job_order_id: &ID,
    ) -> Result<Vec<JobResponse<ID>>>;
}
