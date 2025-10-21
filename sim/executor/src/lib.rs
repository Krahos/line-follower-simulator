use wasm_bindings::devices::TimeUs;
pub use wasmtime;
pub mod mock_stepper;
pub mod wasm_bindings;
pub mod wasm_executor;
pub mod wasm_host;

pub const TOTAL_SIMULATION_TIME_US: TimeUs = 60_000_000;
