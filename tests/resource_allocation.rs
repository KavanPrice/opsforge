use std::collections::HashMap;

use opsforge::ResourceAllocation;
use opsforge::resource_allocation::Allocation;
use opsforge::types::Result;
use rs95::core::{
    equipment::Equipment,
    equipment_hierarchy::{WorkCenter, WorkCenterType, WorkUnit},
    material::MaterialLot,
    operations::OperationsCapability,
    personnel::Person,
    physical_asset::PhysicalAsset,
};
use uuid::Uuid;

struct InMemoryResources {
    equipment: HashMap<Uuid, Equipment<Uuid>>,
    work_centers: HashMap<Uuid, WorkCenter<Uuid>>,
    capabilities: HashMap<Uuid, OperationsCapability<Uuid>>,
    allocations: HashMap<Uuid, Vec<Allocation<Uuid>>>,
    physical_assets: HashMap<Uuid, PhysicalAsset<Uuid>>,
    persons: HashMap<Uuid, Person<Uuid>>,
    material_lots: HashMap<Uuid, MaterialLot<Uuid>>,
}

impl InMemoryResources {
    fn new() -> Self {
        Self {
            equipment: HashMap::new(),
            work_centers: HashMap::new(),
            capabilities: HashMap::new(),
            allocations: HashMap::new(),
            physical_assets: HashMap::new(),
            persons: HashMap::new(),
            material_lots: HashMap::new(),
        }
    }
}

impl ResourceAllocation<Uuid> for InMemoryResources {
    fn get_equipment(&self, id: &Uuid) -> Result<Equipment<Uuid>> {
        self.equipment
            .get(id)
            .cloned()
            .ok_or_else(|| "equipment not found".into())
    }

    fn list_work_centers(
        &self,
        equipment_class_id: Option<&Uuid>,
    ) -> Result<Vec<WorkCenter<Uuid>>> {
        Ok(match equipment_class_id {
            None => self.work_centers.values().cloned().collect(),
            Some(class_id) => self
                .work_centers
                .values()
                .filter(|wc| wc.equipment_classes.contains(class_id))
                .cloned()
                .collect(),
        })
    }

    fn list_work_units(&self, work_center_id: &Uuid) -> Result<Vec<WorkUnit<Uuid>>> {
        Ok(self
            .work_centers
            .get(work_center_id)
            .map(|wc| wc.work_units.clone())
            .unwrap_or_default())
    }

    fn get_capability(&self, id: &Uuid) -> Result<OperationsCapability<Uuid>> {
        self.capabilities
            .get(id)
            .cloned()
            .ok_or_else(|| "capability not found".into())
    }

    fn update_capability(
        &mut self,
        capability: OperationsCapability<Uuid>,
    ) -> Result<OperationsCapability<Uuid>> {
        self.capabilities.insert(capability.id, capability.clone());
        Ok(capability)
    }

    fn allocate(
        &mut self,
        resource_id: Uuid,
        job_order_id: Uuid,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Result<Allocation<Uuid>> {
        let allocation = Allocation {
            resource_id,
            job_order_id,
            start_time,
            end_time,
        };
        self.allocations
            .entry(resource_id)
            .or_default()
            .push(allocation.clone());
        Ok(allocation)
    }

    fn release(&mut self, resource_id: &Uuid, job_order_id: &Uuid) -> Result<()> {
        if let Some(allocs) = self.allocations.get_mut(resource_id) {
            allocs.retain(|a| &a.job_order_id != job_order_id);
        }
        Ok(())
    }

    fn current_allocations(&self, resource_id: &Uuid) -> Result<Vec<Allocation<Uuid>>> {
        Ok(self
            .allocations
            .get(resource_id)
            .cloned()
            .unwrap_or_default())
    }

    fn get_physical_asset(&self, id: &Uuid) -> Result<PhysicalAsset<Uuid>> {
        self.physical_assets
            .get(id)
            .cloned()
            .ok_or_else(|| "physical asset not found".into())
    }

    fn get_person(&self, id: &Uuid) -> Result<Person<Uuid>> {
        self.persons
            .get(id)
            .cloned()
            .ok_or_else(|| "person not found".into())
    }

    fn get_material_lot(&self, id: &Uuid) -> Result<MaterialLot<Uuid>> {
        self.material_lots
            .get(id)
            .cloned()
            .ok_or_else(|| "material lot not found".into())
    }
}

fn make_equipment() -> Equipment<Uuid> {
    Equipment {
        id: Uuid::new_v4(),
        name: "CNC Mill 01".to_string(),
        equipment_classes: vec![],
        properties: vec![],
        sub_equipment: vec![],
    }
}

fn make_work_center() -> WorkCenter<Uuid> {
    WorkCenter {
        id: Uuid::new_v4(),
        name: "Machining Cell A".to_string(),
        work_center_type: WorkCenterType::ProductionUnit,
        equipment_classes: vec![],
        properties: vec![],
        work_units: vec![],
        equipment_capability_test_specifications: vec![],
        equipment_capability_test_results: vec![],
    }
}

#[test]
fn allocate_and_retrieve() {
    let mut ra = InMemoryResources::new();
    let equipment = make_equipment();
    let resource_id = equipment.id;
    ra.equipment.insert(resource_id, equipment);

    let job_order_id = Uuid::new_v4();
    ra.allocate(
        resource_id,
        job_order_id,
        Some("2026-04-14T08:00:00Z".to_string()),
        Some("2026-04-14T12:00:00Z".to_string()),
    )
    .unwrap();

    let allocs = ra.current_allocations(&resource_id).unwrap();
    assert_eq!(allocs.len(), 1);
    assert_eq!(allocs[0].job_order_id, job_order_id);
}

#[test]
fn release_removes_allocation() {
    let mut ra = InMemoryResources::new();
    let resource_id = Uuid::new_v4();
    let job_order_id = Uuid::new_v4();

    ra.allocate(resource_id, job_order_id, None, None).unwrap();
    assert_eq!(ra.current_allocations(&resource_id).unwrap().len(), 1);

    ra.release(&resource_id, &job_order_id).unwrap();
    assert!(ra.current_allocations(&resource_id).unwrap().is_empty());
}

#[test]
fn list_work_centers_unfiltered() {
    let mut ra = InMemoryResources::new();
    let wc1 = make_work_center();
    let wc2 = make_work_center();
    ra.work_centers.insert(wc1.id, wc1);
    ra.work_centers.insert(wc2.id, wc2);

    let result = ra.list_work_centers(None).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn list_work_centers_filtered_by_class() {
    let mut ra = InMemoryResources::new();
    let class_id = Uuid::new_v4();

    let mut wc_with_class = make_work_center();
    wc_with_class.equipment_classes = vec![class_id];
    let wc_without_class = make_work_center();

    ra.work_centers.insert(wc_with_class.id, wc_with_class);
    ra.work_centers.insert(wc_without_class.id, wc_without_class);

    let result = ra.list_work_centers(Some(&class_id)).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].equipment_classes.contains(&class_id));
}

#[test]
fn get_nonexistent_equipment_returns_error() {
    let ra = InMemoryResources::new();
    assert!(ra.get_equipment(&Uuid::new_v4()).is_err());
}
