//! MESA Function 10 — Product Tracking & Genealogy
//!
//! Provides end-to-end visibility into where production units are in the
//! process and a historical record of every operation, component, and
//! measurement associated with a finished product. Supports forward and
//! reverse traceability from raw material through to shipment.
//!
//! ISA-95 models production lots as [`MaterialLot`], sublots as
//! [`MaterialSublot`], and component linkage via the `assembled_from` field.
//! Execution history is captured through [`SegmentResponse`] and
//! [`OperationsResponse`].

use crate::types::Result;
use rs95::core::{
    material::{MaterialLot, MaterialSublot},
    operations::{OperationsResponse, SegmentResponse},
};

/// Core interface for product tracking and genealogy (MESA Function 10).
pub trait ProductTracking<ID> {
    /// Return a material lot by ID.
    fn get_lot(&self, id: &ID) -> Result<MaterialLot<ID>>;

    /// Return a material sublot by ID.
    fn get_sublot(&self, id: &ID) -> Result<MaterialSublot<ID>>;

    /// Return all lots associated with a job order.
    fn lots_for_job_order(&self, job_order_id: &ID) -> Result<Vec<MaterialLot<ID>>>;

    /// Return all sublots belonging to a lot.
    fn sublots_for_lot(&self, lot_id: &ID) -> Result<Vec<MaterialSublot<ID>>>;

    /// Record a segment response as a traceability event for the lots it consumed
    /// or produced.
    fn record_segment_response(&mut self, response: SegmentResponse<ID>) -> Result<()>;

    /// Return all segment responses that reference a given lot.
    fn history_for_lot(&self, lot_id: &ID) -> Result<Vec<SegmentResponse<ID>>>;

    /// Return the full operations response for a job order (forward traceability).
    fn operations_response_for_job(
        &self,
        job_order_id: &ID,
    ) -> Result<OperationsResponse<ID>>;

    /// Return all lots that were consumed to produce a given lot
    /// (reverse genealogy — the `assembled_from` graph).
    fn component_lots(&self, lot_id: &ID) -> Result<Vec<MaterialLot<ID>>>;
}
