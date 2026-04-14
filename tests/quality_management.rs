use std::collections::HashMap;

use opsforge::QualityManagement;
use opsforge::quality_management::{Disposition, NonConformanceReport};
use opsforge::types::Result;
use rs95::core::{
    equipment::{EquipmentCapabilityTestResult, EquipmentCapabilityTestSpecification},
    material::{MaterialTestSpecification, QATestResult},
};
use uuid::Uuid;

struct InMemoryQuality {
    test_specs: HashMap<Uuid, MaterialTestSpecification<Uuid>>,
    test_results: HashMap<Uuid, QATestResult<Uuid>>,
    equipment_specs: HashMap<Uuid, EquipmentCapabilityTestSpecification<Uuid>>,
    equipment_results: HashMap<Uuid, EquipmentCapabilityTestResult<Uuid>>,
    ncrs: HashMap<Uuid, NonConformanceReport<Uuid>>,
}

impl InMemoryQuality {
    fn new() -> Self {
        Self {
            test_specs: HashMap::new(),
            test_results: HashMap::new(),
            equipment_specs: HashMap::new(),
            equipment_results: HashMap::new(),
            ncrs: HashMap::new(),
        }
    }
}

impl QualityManagement<Uuid> for InMemoryQuality {
    fn get_material_test_spec(&self, id: &Uuid) -> Result<MaterialTestSpecification<Uuid>> {
        self.test_specs
            .get(id)
            .cloned()
            .ok_or_else(|| "test spec not found".into())
    }

    fn record_material_test(&mut self, result: QATestResult<Uuid>) -> Result<QATestResult<Uuid>> {
        self.test_results.insert(result.id, result.clone());
        Ok(result)
    }

    fn results_for_lot(&self, lot_id: &Uuid) -> Result<Vec<QATestResult<Uuid>>> {
        Ok(self
            .test_results
            .values()
            .filter(|r| r.material_lot_property_ids.contains(lot_id))
            .cloned()
            .collect())
    }

    fn get_equipment_capability_spec(
        &self,
        id: &Uuid,
    ) -> Result<EquipmentCapabilityTestSpecification<Uuid>> {
        self.equipment_specs
            .get(id)
            .cloned()
            .ok_or_else(|| "equipment capability spec not found".into())
    }

    fn record_equipment_capability(
        &mut self,
        result: EquipmentCapabilityTestResult<Uuid>,
    ) -> Result<EquipmentCapabilityTestResult<Uuid>> {
        self.equipment_results.insert(
            result.equipment_capability_test_specification_id,
            result.clone(),
        );
        Ok(result)
    }

    fn raise_ncr(
        &mut self,
        ncr: NonConformanceReport<Uuid>,
    ) -> Result<NonConformanceReport<Uuid>> {
        self.ncrs.insert(ncr.id, ncr.clone());
        Ok(ncr)
    }

    fn close_ncr(
        &mut self,
        ncr_id: &Uuid,
        disposition: Disposition,
        closed_at: Option<String>,
    ) -> Result<NonConformanceReport<Uuid>> {
        let ncr = self.ncrs.get_mut(ncr_id).ok_or("NCR not found")?;
        ncr.disposition = Some(disposition);
        ncr.closed_at = closed_at;
        Ok(ncr.clone())
    }

    fn open_ncrs(&self) -> Result<Vec<NonConformanceReport<Uuid>>> {
        Ok(self
            .ncrs
            .values()
            .filter(|n| n.disposition.is_none())
            .cloned()
            .collect())
    }
}

fn make_ncr(job_order_id: Uuid) -> NonConformanceReport<Uuid> {
    NonConformanceReport {
        id: Uuid::new_v4(),
        job_order_id,
        material_lot_id: None,
        description: "Surface finish out of tolerance".to_string(),
        disposition: None,
        raised_at: Some("2026-04-14T09:00:00Z".to_string()),
        closed_at: None,
    }
}

#[test]
fn raise_ncr_appears_in_open_ncrs() {
    let mut qm = InMemoryQuality::new();
    let ncr = make_ncr(Uuid::new_v4());

    qm.raise_ncr(ncr).unwrap();

    let open = qm.open_ncrs().unwrap();
    assert_eq!(open.len(), 1);
    assert!(open[0].disposition.is_none());
}

#[test]
fn close_ncr_removes_from_open() {
    let mut qm = InMemoryQuality::new();
    let ncr = make_ncr(Uuid::new_v4());
    let ncr_id = ncr.id;

    qm.raise_ncr(ncr).unwrap();
    qm.close_ncr(
        &ncr_id,
        Disposition::Scrap,
        Some("2026-04-14T11:00:00Z".to_string()),
    )
    .unwrap();

    let open = qm.open_ncrs().unwrap();
    assert!(open.is_empty());
}

#[test]
fn closed_ncr_has_correct_disposition() {
    let mut qm = InMemoryQuality::new();
    let ncr = make_ncr(Uuid::new_v4());
    let ncr_id = ncr.id;

    qm.raise_ncr(ncr).unwrap();
    let closed = qm
        .close_ncr(&ncr_id, Disposition::Rework, None)
        .unwrap();

    assert_eq!(closed.disposition, Some(Disposition::Rework));
}

#[test]
fn record_and_retrieve_material_test_result() {
    let mut qm = InMemoryQuality::new();
    let spec_id = Uuid::new_v4();

    let result = QATestResult {
        id: Uuid::new_v4(),
        name: "Tensile strength test".to_string(),
        material_test_specification_id: spec_id,
        material_lot_property_ids: vec![],
    };

    let recorded = qm.record_material_test(result).unwrap();
    assert_eq!(recorded.material_test_specification_id, spec_id);
}
