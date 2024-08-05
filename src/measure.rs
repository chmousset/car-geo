use dioxus::prelude::*;

struct Config {
    pub wheelbase: Signal<f64>,
    pub front_overhang: Signal<f64>,
    pub read_overhang: Signal<f64>,
    pub front_laser_width: Signal<f64>,
    pub back_laser_width: Signal<f64>,
    pub laser_angle: Signal<f64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wheelbase: use_signal(|| 2.420),
            front_overhang: use_signal(|| (4.181 - 2.420) * 0.5),
            read_overhang: use_signal(|| (4.181 - 2.420) * 0.5),
            front_laser_width: use_signal(|| 1.980),
            back_laser_width: use_signal(|| 1.980),
            laser_angle: use_signal(|| 0.0),
        }
    }
}

impl Config {
    pub fn calc(&mut self) {
        let measure_lenght =
            *self.wheelbase.read() + *self.front_overhang.read() + *self.read_overhang.read();
        let measure_width_diff = *self.back_laser_width.read() - *self.front_laser_width.read();
        let tan = measure_width_diff / measure_lenght;
        *self.laser_angle.write() = tan.atan();
    }
}

struct WheelMeasure {
    pub front_measure: Signal<f64>,
    pub back_measure: Signal<f64>,
    pub top_measure: Signal<f64>,
    pub bottom_measure: Signal<f64>,
    pub camber: Signal<f64>,
    pub toe: Signal<f64>,
}

struct CarMeasure {
    front_right: WheelMeasure,
    front_left: WheelMeasure,
    back_right: WheelMeasure,
    back_left: WheelMeasure,
}

impl WheelMeasure {
    pub fn calc(&mut self, laser_angle: Signal<f64>) {
        let laser = *laser_angle.read();
    }
}
