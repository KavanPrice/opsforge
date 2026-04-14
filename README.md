# opsforge

A Rust library defining the behavioural contracts for a **MESA**-compliant Manufacturing Operations Management (MOM) system.

This library provides trait definitions for each of the eleven functions described in the MESA International MOM standard: resource allocation, scheduling, dispatching, document control, data collection, labour management, quality management, process management, maintenance management, product tracking, and performance analysis. By implementing these traits, you effectively implement a standards-compliant MES.

Domain objects passed through these traits — equipment, materials, personnel, job orders, and so on — are modelled by the companion [`rs95`](https://crates.io/crates/rs95) crate, which implements the ISA-95 object model. The two crates are designed to be used together.

## Features and Details

All traits are parameterised by an `ID` type, allowing you to use `Uuid`, `String`, `u64`, or any custom type as identifiers. Supporting types defined in opsforge (such as `Allocation`, `ProcessAlarm`, and `NonConformanceReport`) follow the same convention.

The `default-models` feature provides concrete type aliases for all opsforge-owned types using `uuid::Uuid` as the identifier, removing the need to write generic parameters in most application code.

Optional `Serialize` and `Deserialize` support is available via the `serde` feature flag. Enabling it also activates `rs95/serde`, so both crates' types are covered with a single flag.

## Usage

### Implementing a Trait

Implement any of the eleven MESA function traits for your own MES backend:

```rust
use opsforge::Dispatching;
use rs95::core::operations::{JobOrder, JobOrderCommandType, JobOrderStatus, OperationsResponse};

struct MyMes;

impl Dispatching<uuid::Uuid> for MyMes {
    fn issue(&mut self, order: JobOrder<uuid::Uuid>) -> opsforge::types::Result<JobOrder<uuid::Uuid>> {
        // persist and activate the job order
        todo!()
    }

    fn send_command(
        &mut self,
        job_order_id: &uuid::Uuid,
        command: JobOrderCommandType,
    ) -> opsforge::types::Result<JobOrder<uuid::Uuid>> {
        todo!()
    }

    // ...remaining methods
}
```

### Using Default Models

Enable the `default-models` feature to import concrete aliases without generic parameters:

```rust
use opsforge::default_models::{Allocation, ProcessAlarm};
use rs95::default_models::operations::JobOrder;
```

```toml
[dependencies]
opsforge = { version = "0.1.0", features = ["default-models"] }
rs95 = "0.2.0"
```

### Using a Custom ID Type

If your system uses a non-UUID identifier (e.g. a URI or an integer), use the generic traits and types directly:

```rust
use opsforge::resource_allocation::{Allocation, ResourceAllocation};

let allocation = Allocation::<String> {
    resource_id: "http://factory.com/assets/cnc-01".to_string(),
    job_order_id: "JO-2026-001".to_string(),
    start_time: Some("2026-04-14T08:00:00Z".to_string()),
    end_time: Some("2026-04-14T12:00:00Z".to_string()),
};
```

## Feature Flags

- `default-models`: Provides concrete type aliases using `uuid::Uuid` in the `opsforge::default_models` module.
- `serde`: Enables `Serialize` and `Deserialize` for all opsforge-owned types, and enables `rs95/serde` so ISA-95 types are covered too.

```toml
[dependencies]
opsforge = { version = "0.1.0", features = ["default-models", "serde"] }
```
