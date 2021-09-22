use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const DWD_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/hourly/air_temperature/historical/";
const STATION_DESCR_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/hourly/air_temperature/historical/TU_Stundenwerte_Beschreibung_Stationen.txt";

fn filter_station_info(station_info: &str, station_name: &str) -> Result<String> {
    let station_dir_re = Regex::new(r"(?P<station_id>[0-9]{5})\s+(?P<from>[0-9]{8})\s+(?P<until>[0-9]{8})\s+(?P<elevation>-?[0-9]{1,4})\s+(?P<lat>[45][0-9]\.[0-9]{4})\s+(?P<lon>[1]?[0-9]\.[0-9]{4})\s+(?P<station_name>[A-ZÄ-Ü].*\S)\s+(?P<state>[A-Z].*\s+)").unwrap();
    let station_match = station_dir_re
        .captures_iter(station_info)
        .filter(|c| &c["station_name"] == station_name)
        .take(1)
        .collect::<Vec<_>>();
    Ok(station_match[0]["station_id"].to_owned())
}

pub async fn get_station_id_from_name(station_name: &str) -> Result<String> {
    let response = reqwest::get(STATION_DESCR_URL).await?;
    let station_info = response.text_with_charset("ascii").await?;
    let station_info = station_info.split('\n').skip(2).collect::<String>();
    filter_station_info(&station_info, station_name)
}

pub async fn download_zip_archive(download_path: &Path, station_id: &str) -> Result<PathBuf> {
    let response = reqwest::get(DWD_URL).await?;
    let toc = response.text().await?;
    let zipname = find_station_zipfile(&toc, station_id);
    let fname = download_path.join(&zipname);
    let mut dest = File::create(&fname)?;
    let download_url = format!("{}/{}", DWD_URL, zipname);
    let download_response = reqwest::get(download_url).await?;
    let content = download_response.bytes().await?;
    dest.write_all(&content)?;
    Ok(fname)
}

fn find_station_zipfile(toc: &str, station_id: &str) -> String {
    let temperature_hourly_file_re = Regex::new(
        r"(?P<file_name>stundenwerte_TU_(?P<station_id>[0-9]{5})_(?:akt|(?:[0-9]{8}_[0-9]{8}_hist)).zip)</a>",
    ).unwrap();
    let zip = temperature_hourly_file_re
        .captures_iter(toc)
        .filter(|c| &c["station_id"] == station_id)
        .take(1)
        .collect::<Vec<_>>();
    zip[0]["file_name"].to_owned()
}

#[cfg(test)]
mod find_station_zipfile_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_correct_toc_small() {
        let fake_toc = "stundenwerte_TU_00044_20070401_20201231_hist.zip</a>\n";
        let correct_res = "stundenwerte_TU_00044_20070401_20201231_hist.zip";
        let res = find_station_zipfile(fake_toc, "00044");
        assert_eq!(res, correct_res);
    }

    #[test]
    fn test_correct_toc_big() {
        let fake_toc = "stundenwerte_TU_00044_20070401_20201231_hist.zip</a>\n
        stundenwerte_TU_00055_20070401_20201231_hist.zip</a>\n
        stundenwerte_TU_00066_20070401_20201231_hist.zip</a>\n";
        let correct_res = "stundenwerte_TU_00044_20070401_20201231_hist.zip";
        let res = find_station_zipfile(fake_toc, "00044");
        assert_eq!(res, correct_res);
    }

    #[test]
    #[should_panic]
    fn test_wrong_toc() {
        let fake_toc = "xxxx\n";
        let _res = find_station_zipfile(fake_toc, "00044");
    }
}

#[cfg(test)]
mod filter_station_info_test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first_line() {
        let station_file = "00003 19500401 20110331            202     50.7827    6.0941 Aachen                                   Nordrhein-Westfalen                                                                               
        00044 20070401 20210920             44     52.9336    8.2370 Großenkneten                             Niedersachsen                                                                                     
        00052 19760101 19880101             46     53.6623   10.1990 Ahrensburg-Wulfsdorf                     Schleswig-Holstein ";
        let res = filter_station_info(station_file, "Aachen");
        assert_eq!(res.unwrap(), "00003");
    }

    #[test]
    fn test_second_line() {
        let station_file = "00003 19500401 20110331            202     50.7827    6.0941 Aachen                                   Nordrhein-Westfalen                                                                               
        00044 20070401 20210920             44     52.9336    8.2370 Großenkneten                             Niedersachsen                                                                                     
        00052 19760101 19880101             46     53.6623   10.1990 Ahrensburg-Wulfsdorf                     Schleswig-Holstein ";
        let res = filter_station_info(station_file, "Großenkneten");
        assert_eq!(res.unwrap(), "00044");
    }

    #[test]
    fn test_third_line() {
        let station_file = "00003 19500401 20110331            202     50.7827    6.0941 Aachen                                   Nordrhein-Westfalen                                                                               
        00044 20070401 20210920             44     52.9336    8.2370 Großenkneten                             Niedersachsen                                                                                     
        00052 19760101 19880101             46     53.6623   10.1990 Ahrensburg-Wulfsdorf                     Schleswig-Holstein ";
        let res = filter_station_info(station_file, "Ahrensburg-Wulfsdorf");
        assert_eq!(res.unwrap(), "00052");
    }
}
