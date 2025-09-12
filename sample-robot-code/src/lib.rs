#[allow(warnings)]
mod bindings;

use bindings::devices::{DeviceOperation, device_operation_blocking, set_motors_power};
use bindings::exports::robot::{Color, Configuration, Guest};

struct Component;

impl Guest for Component {
    fn setup() -> Configuration {
        Configuration {
            name: "Liner".to_string(),
            color_main: Color { r: 255, g: 0, b: 0 },
            color_secondary: Color { r: 0, g: 255, b: 0 },
            width_axle: 200.0,
            length_front: 300.0,
            length_back: 20.0,
            clearing_back: 3.0,
            wheel_diameter: 15.0,
            gear_ratio_num: 1,
            gear_ratio_den: 20,
            front_sensors_spacing: 4.0,
            front_sensors_height: 4.0,
        }
    }

    fn run() -> () {
        loop {
            set_motors_power(0, 0);
            device_operation_blocking(DeviceOperation::SleepFor(10000));
        }
    }
}

bindings::export!(Component with_types_in bindings);
