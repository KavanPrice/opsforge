use std::collections::HashMap;

use opsforge::Dispatching;
use opsforge::types::Result;
use rs95::core::operations::{
    JobOrder, JobOrderCommandType, JobOrderStatus, OperationType, OperationsResponse,
    OperationsResponseResult,
};
use uuid::Uuid;

struct InMemoryMes {
    job_orders: HashMap<Uuid, JobOrder<Uuid>>,
    responses: HashMap<Uuid, OperationsResponse<Uuid>>,
}

impl InMemoryMes {
    fn new() -> Self {
        Self {
            job_orders: HashMap::new(),
            responses: HashMap::new(),
        }
    }
}

impl Dispatching<Uuid> for InMemoryMes {
    fn issue(&mut self, order: JobOrder<Uuid>) -> Result<JobOrder<Uuid>> {
        self.job_orders.insert(order.id, order.clone());
        Ok(order)
    }

    fn send_command(
        &mut self,
        job_order_id: &Uuid,
        command: JobOrderCommandType,
    ) -> Result<JobOrder<Uuid>> {
        let order = self
            .job_orders
            .get_mut(job_order_id)
            .ok_or("job order not found")?;
        order.command = command;
        Ok(order.clone())
    }

    fn get_job_order(&self, id: &Uuid) -> Result<JobOrder<Uuid>> {
        self.job_orders
            .get(id)
            .cloned()
            .ok_or_else(|| "job order not found".into())
    }

    fn list_by_status(&self, status: JobOrderStatus) -> Result<Vec<JobOrder<Uuid>>> {
        Ok(self
            .job_orders
            .values()
            .filter(|o| o.status == status)
            .cloned()
            .collect())
    }

    fn record_response(
        &mut self,
        response: OperationsResponse<Uuid>,
    ) -> Result<OperationsResponse<Uuid>> {
        self.responses.insert(response.id, response.clone());
        Ok(response)
    }

    fn get_response(&self, job_order_id: &Uuid) -> Result<OperationsResponse<Uuid>> {
        self.responses
            .get(job_order_id)
            .cloned()
            .ok_or_else(|| "response not found".into())
    }
}

fn make_job_order() -> JobOrder<Uuid> {
    JobOrder {
        id: Uuid::new_v4(),
        work_type: OperationType::Production,
        command: JobOrderCommandType::Start,
        status: JobOrderStatus::Ready,
        priority: Some(1),
        operations_request_id: None,
        operations_definition_id: None,
        start_time: Some("2026-04-14T08:00:00Z".to_string()),
        end_time: None,
        parameters: vec![],
    }
}

#[test]
fn issue_and_retrieve_job_order() {
    let mut mes = InMemoryMes::new();
    let order = make_job_order();
    let id = order.id;

    mes.issue(order).unwrap();

    let retrieved = mes.get_job_order(&id).unwrap();
    assert_eq!(retrieved.id, id);
    assert_eq!(retrieved.status, JobOrderStatus::Ready);
}

#[test]
fn send_command_updates_job_order() {
    let mut mes = InMemoryMes::new();
    let order = make_job_order();
    let id = order.id;
    mes.issue(order).unwrap();

    let updated = mes.send_command(&id, JobOrderCommandType::Stop).unwrap();
    assert_eq!(updated.command, JobOrderCommandType::Stop);
}

#[test]
fn list_by_status_filters_correctly() {
    let mut mes = InMemoryMes::new();

    let ready = make_job_order();
    let mut running = make_job_order();
    running.status = JobOrderStatus::Running;

    mes.issue(ready).unwrap();
    mes.issue(running).unwrap();

    let ready_orders = mes.list_by_status(JobOrderStatus::Ready).unwrap();
    assert_eq!(ready_orders.len(), 1);
    assert_eq!(ready_orders[0].status, JobOrderStatus::Ready);
}

#[test]
fn record_and_retrieve_response() {
    let mut mes = InMemoryMes::new();
    let order = make_job_order();
    let order_id = order.id;
    mes.issue(order).unwrap();

    let response = OperationsResponse {
        id: order_id,
        operations_request_id: Uuid::new_v4(),
        actual_start_time: Some("2026-04-14T08:00:00Z".to_string()),
        actual_end_time: Some("2026-04-14T10:00:00Z".to_string()),
        result: OperationsResponseResult::Completed,
        segment_responses: vec![],
        job_responses: vec![],
    };

    mes.record_response(response).unwrap();

    let retrieved = mes.get_response(&order_id).unwrap();
    assert_eq!(retrieved.result, OperationsResponseResult::Completed);
}

#[test]
fn get_nonexistent_job_order_returns_error() {
    let mes = InMemoryMes::new();
    assert!(mes.get_job_order(&Uuid::new_v4()).is_err());
}
