//! MESA Function 4 — Document Control
//!
//! Manages the controlled documents that accompany manufacturing operations:
//! work instructions, recipes, engineering drawings, SOPs, and forms.
//! Ensures operators work from the correct, approved revision.
use crate::types::{DocumentId, OperationId, Result, Timestamp, WorkOrderId};

/// Revision lifecycle state of a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentStatus {
    Draft,
    InReview,
    Approved,
    Obsolete,
}

/// A controlled document and its current revision metadata.
#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub revision: String,
    pub status: DocumentStatus,
    pub content_ref: String,
    pub approved_at: Option<Timestamp>,
    pub approved_by: Option<String>,
}

/// Core interface for document control (MESA Function 4).
pub trait DocumentControl {
    /// Retrieve a specific revision of a document.
    fn get(&self, id: &DocumentId) -> Result<Document>;

    /// List all documents associated with a work order or operation.
    fn list_for_operation(&self, operation_id: &OperationId) -> Result<Vec<Document>>;

    /// List all documents associated with a work order.
    fn list_for_work_order(&self, work_order_id: &WorkOrderId) -> Result<Vec<Document>>;

    /// Create a new draft document. Returns the created [`Document`].
    fn create(&mut self, title: &str, content_ref: &str) -> Result<Document>;

    /// Submit a document for approval review.
    fn submit_for_review(&mut self, id: &DocumentId) -> Result<Document>;

    /// Approve a document at the given revision, making it effective.
    fn approve(&mut self, id: &DocumentId, approved_by: &str, at: Timestamp) -> Result<Document>;

    /// Obsolete an approved document, preventing further use.
    fn obsolete(&mut self, id: &DocumentId) -> Result<Document>;

    /// Associate a document with a specific operation.
    fn attach_to_operation(
        &mut self,
        document_id: &DocumentId,
        operation_id: &OperationId,
    ) -> Result<()>;
}
