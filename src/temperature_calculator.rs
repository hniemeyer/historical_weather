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
    let mut skipped_years: i32 = 0;
    for act_year in min_year..=max_year {
        let res = measurement_vec
            .iter()
            .filter(|x| x.date.date() == NaiveDate::from_ymd(act_year, month, day))
            .map(|x| OrderedFloat(x.measurement))
            .max();
        match res {
            Some(x) => max_temp += x.into_inner(),
            None => {
                skipped_years += 1;
                continue;
            }
        }
        let res2 = measurement_vec
            .iter()
            .filter(|x| x.date.date() == NaiveDate::from_ymd(act_year, month, day))
            .map(|x| OrderedFloat(x.measurement))
            .min()
            .unwrap();
        min_temp += res2.into_inner();
    }
    let number_of_years = max_year - min_year + 1 - skipped_years;
    (
        min_temp / number_of_years as f64,
        max_temp / number_of_years as f64,
    )
}

#[cfg(test)]
mod min_max_year_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_correct_two_years() {
        let fake_date = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 5.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2019, 1, 1).and_hms(10, 0, 0),
                measurement: 5.0,
            },
        ];
        let (a, b) = get_min_max_year(&fake_date);
        assert_eq!(a, 2019);
        assert_eq!(b, 2020);
    }

    #[test]
    fn test_correct_same_year() {
        let fake_date = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2019, 1, 1).and_hms(10, 0, 0),
                measurement: 5.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2019, 1, 1).and_hms(10, 0, 0),
                measurement: 5.0,
            },
        ];
        let (a, b) = get_min_max_year(&fake_date);
        assert_eq!(a, 2019);
        assert_eq!(b, 2019);
    }

    #[test]
    fn test_correct_one_year() {
        let fake_date = vec![TemperatureMeasurement {
            date: NaiveDate::from_ymd(2019, 1, 1).and_hms(10, 0, 0),
            measurement: 5.0,
        }];
        let (a, b) = get_min_max_year(&fake_date);
        assert_eq!(a, 2019);
        assert_eq!(b, 2019);
    }
}
