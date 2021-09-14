use std::fmt;

pub struct TemperatureMeasurement {
    date: String,
    measurement: f32,
}

impl TemperatureMeasurement {
    pub fn new(date: String, measurement: f32) -> Self {
        Self { date, measurement }
    }
}

impl fmt::Display for TemperatureMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date={}, temperature={}", self.date, self.measurement)
    }
}
