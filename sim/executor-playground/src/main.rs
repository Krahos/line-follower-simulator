use bindings::devices::TimeUs;

pub mod bindings;
pub mod bot_executor;
pub mod bot_wasm_host;
pub mod mock_stepper;

const TOTAL_SIMULATION_TIME_US: TimeUs = 60_000_000;

fn main() -> wasmtime::Result<()> {
    // Load the component from disk
    let wasm_bytes = std::fs::read("../bot/target/wasm32-wasip1/release/line_follower_robot.wasm")?;

    // Create a mock stepper
    let stepper = mock_stepper::MockStepper::new();

    // Create the bot executor
    let bot_executor =
        bot_executor::BotExecutor::new(&wasm_bytes, stepper, TOTAL_SIMULATION_TIME_US)?;

    println!(
        "Robot configuration: {:#?}",
        bot_executor.robot_configuration()
    );

    Ok(())
}
