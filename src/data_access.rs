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
        let datetime = parse_date_string(&date_string)?;

        measurement_vec.push(TemperatureMeasurement::new(
            datetime,
            record.get(3).unwrap().trim().parse::<f32>()?,
        ));
    }

    Ok(measurement_vec)
}

fn parse_date_string(datetime_string: &str) -> Result<NaiveDateTime> {
    let year = datetime_string
        .chars()
        .take(4)
        .collect::<String>()
        .parse::<i32>()?;
    let month = datetime_string
        .chars()
        .skip(4)
        .take(2)
        .collect::<String>()
        .parse::<u32>()?;
    let day = datetime_string
        .chars()
        .skip(6)
        .take(2)
        .collect::<String>()
        .parse::<u32>()?;
    let hour = datetime_string
        .chars()
        .skip(8)
        .take(2)
        .collect::<String>()
        .parse::<u32>()?;
    Ok(NaiveDate::from_ymd(year, month, day).and_hms(hour, 0, 0))
}

#[cfg(test)]
mod data_access_tests {
    use chrono::NaiveTime;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_correct_string() {
        let fake_date = "2020010110".to_owned();
        let res = parse_date_string(&fake_date).unwrap();
        assert_eq!(res.date(), NaiveDate::from_ymd(2020, 1, 1));
        assert_eq!(res.time(), NaiveTime::from_hms(10, 0, 0));
    }
}
