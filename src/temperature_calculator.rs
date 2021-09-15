use crate::data_access::TemperatureMeasurement;
use chrono::prelude::*;
use chrono::NaiveDate;
use ordered_float::OrderedFloat;

fn get_min_max_year(measurement_vec: &[TemperatureMeasurement]) -> (i32, i32) {
    let min_year = measurement_vec
        .iter()
        .map(|x| x.date.year())
        .min()
        .unwrap_or(0);
    let max_year = measurement_vec
        .iter()
        .map(|x| x.date.year())
        .max()
        .unwrap_or(0);
    (min_year, max_year)
}

pub fn get_average_temperatures(
    measurement_vec: &[TemperatureMeasurement],
    day: u32,
    month: u32,
) -> (f64, f64) {
    let (min_year, max_year) = get_min_max_year(measurement_vec);

    let mut max_temp = 0.0;
    let mut min_temp = 0.0;
    for act_year in min_year..=max_year {
        //TODO: Skip missing data (dont unwrap, but match)
        let res = measurement_vec
            .iter()
            .filter(|x| x.date.date() == NaiveDate::from_ymd(act_year, month, day))
            .map(|x| OrderedFloat(x.measurement))
            .max()
            .unwrap();
        max_temp += res.into_inner();
        let res2 = measurement_vec
            .iter()
            .filter(|x| x.date.date() == NaiveDate::from_ymd(act_year, month, day))
            .map(|x| OrderedFloat(x.measurement))
            .min()
            .unwrap();
        min_temp += res2.into_inner();
    }
    let number_of_years = max_year - min_year + 1;
    (
        min_temp / number_of_years as f64,
        max_temp / number_of_years as f64,
    )
}
