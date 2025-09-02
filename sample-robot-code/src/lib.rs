#[allow(warnings)]
mod bindings;

use bindings::exports::robot::{Color, Configuration, Guest};
use bindings::motors::*;
use bindings::sensors::*;

struct Component;

impl Guest for Component {
    fn setup() -> Configuration {
        Configuration {
            name: "Liner".to_string(),
            color_main: Color { r: 255, g: 0, b: 0 },
            color_secondary: Color { r: 0, g: 255, b: 0 },
            width_axle: 200,
            length_front: 300,
            length_back: 20,
            clearing_back: 3,
            wheel_radius: 15,
            gear_ratio_num: 1,
            gear_ratio_den: 20,
            front_sensors_spacing: 4,
            front_sensors_height: 4,
        }
    }

    fn run() -> () {
        loop {
            set_power(0, 0);
            sleep_blocking_for(10000);
        }
    }
}

bindings::export!(Component with_types_in bindings);
