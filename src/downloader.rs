use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub async fn download_zip_archive(download_path: &Path, station_id: &str) -> Result<PathBuf> {
    let dwd_url = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/hourly/air_temperature/historical/";
        let response = reqwest::get(dwd_url).await?;
    let toc = response.text().await?;
    let zipname = find_station_zipfile(&toc, station_id);
    let fname = download_path.join(&zipname);
    let mut dest = File::create(&fname)?;
    let download_url = format!("{}/{}", dwd_url, zipname);
    let download_response = reqwest::get(download_url).await?;
    let content = download_response.bytes().await?;
    dest.write_all(&content)?;
    Ok(fname)

}

fn find_station_zipfile(toc: &str, station_id: &str) -> String {
    let temperature_hourly_file_re = Regex::new(
        r"(?P<file_name>stundenwerte_TU_(?P<station_id>[0-9]{5})_(?:akt|(?:[0-9]{8}_[0-9]{8}_hist)).zip)</a>",
    ).unwrap();
    let zip = temperature_hourly_file_re.captures_iter(toc).filter(|c| &c["station_id"] == station_id).take(1).collect::<Vec<_>>();
    zip[0]["file_name"].to_owned()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_correct_toc() {
        let fake_toc = "stundenwerte_TU_00044_20070401_20201231_hist.zip</a>\n";
        let correct_res = "stundenwerte_TU_00044_20070401_20201231_hist.zip";
        let res = find_station_zipfile(fake_toc, "00044");
        assert_eq!(res, correct_res);
    }

  
}
