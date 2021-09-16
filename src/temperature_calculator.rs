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

fn get_min_max_temp_at_date(
    measurement_vec: &[TemperatureMeasurement],
    date: NaiveDate,
) -> (Option<f64>, Option<f64>) {
    let res_max = measurement_vec
        .iter()
        .filter(|x| x.date.date() == date)
        .map(|x| OrderedFloat(x.measurement))
        .max();
    let return_val_max = res_max.map(|x| x.into_inner());
    let res_min = measurement_vec
        .iter()
        .filter(|x| x.date.date() == date)
        .map(|x| OrderedFloat(x.measurement))
        .min();
    let return_val_min = res_min.map(|x| x.into_inner());
    (return_val_min, return_val_max)
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
        let (min_opt, max_opt) =
            get_min_max_temp_at_date(measurement_vec, NaiveDate::from_ymd(act_year, month, day));
        match (min_opt, max_opt) {
            (Some(x), Some(y)) => {
                max_temp += y;
                min_temp += x;
            }
            (None, None) => {
                skipped_years += 1;
                continue;
            }
            _ => panic!("This should not happen."),
        }
    }
    let number_of_years = max_year - min_year + 1 - skipped_years;
    if number_of_years == 0 {
        panic!("No data for given date available.");
    }
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

#[cfg(test)]
mod min_max_temp_at_date_test {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_correct_two_different_values() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 15.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
        ];
        let (a, b) = get_min_max_temp_at_date(&fake_data, NaiveDate::from_ymd(2020, 1, 1));
        assert_relative_eq!(a.unwrap(), 6.0);
        assert_relative_eq!(b.unwrap(), 15.0);
    }

    #[test]
    fn test_correct_two_same_values() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 6.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
        ];
        let (a, b) = get_min_max_temp_at_date(&fake_data, NaiveDate::from_ymd(2020, 1, 1));
        assert_relative_eq!(a.unwrap(), 6.0);
        assert_relative_eq!(b.unwrap(), 6.0);
    }

    #[test]
    fn test_wrong_date_equals_none() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 15.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
        ];
        let (a, b) = get_min_max_temp_at_date(&fake_data, NaiveDate::from_ymd(2019, 1, 1));
        assert!(!a.is_some());
        assert!(!b.is_some());
    }
}

#[cfg(test)]
mod average_temp_at_day_tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_only_one_year() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 15.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
        ];
        let (a, b) = get_average_temperatures(&fake_data, 1, 1);
        assert_relative_eq!(a, 6.0);
        assert_relative_eq!(b, 15.0);
    }

    #[test]
    #[should_panic]
    fn test_only_no_year() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 10, 1).and_hms(10, 0, 0),
                measurement: 15.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 10, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
        ];
        let (_a, _b) = get_average_temperatures(&fake_data, 1, 1);
    }

    #[test]
    fn test_with_skipped_year() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 15.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2019, 10, 10).and_hms(11, 0, 0),
                measurement: 6.0,
            },
        ];
        let (a, b) = get_average_temperatures(&fake_data, 1, 1);
        assert_relative_eq!(a, 6.0);
        assert_relative_eq!(b, 15.0);
    }

    #[test]
    fn test_with_two_years() {
        let fake_data = vec![
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 0, 0),
                measurement: 15.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2020, 1, 1).and_hms(11, 0, 0),
                measurement: 6.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2019, 1, 1).and_hms(11, 0, 0),
                measurement: 3.0,
            },
            TemperatureMeasurement {
                date: NaiveDate::from_ymd(2019, 1, 1).and_hms(10, 0, 0),
                measurement: 20.0,
            },
        ];
        let (a, b) = get_average_temperatures(&fake_data, 1, 1);
        assert_relative_eq!(a, 4.5);
        assert_relative_eq!(b, 17.5);
    }

}
