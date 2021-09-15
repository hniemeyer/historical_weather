use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt;
use std::path::Path;

pub struct TemperatureMeasurement {
    date: NaiveDateTime,
    measurement: f32,
}

impl TemperatureMeasurement {
    pub fn new(date: NaiveDateTime, measurement: f32) -> Self {
        Self { date, measurement }
    }
}

impl fmt::Display for TemperatureMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date={}, temperature={}", self.date, self.measurement)
    }
}

pub fn load_data(path: &Path) -> Result<Vec<TemperatureMeasurement>> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(path)?;

    let mut measurement_vec = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let date_string = record.get(1).unwrap().to_string();
        let year = date_string
            .chars()
            .take(4)
            .collect::<String>()
            .parse::<i32>()?;
        let month = date_string
            .chars()
            .skip(4)
            .take(2)
            .collect::<String>()
            .parse::<u32>()?;
        let day = date_string
            .chars()
            .skip(6)
            .take(2)
            .collect::<String>()
            .parse::<u32>()?;
        let hour = date_string
            .chars()
            .skip(8)
            .take(2)
            .collect::<String>()
            .parse::<u32>()?;

        measurement_vec.push(TemperatureMeasurement::new(
            NaiveDate::from_ymd(year, month, day).and_hms(hour, 0, 0),
            record.get(3).unwrap().trim().parse::<f32>()?,
        ));
    }

    Ok(measurement_vec)
}
