use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    let dwd_url = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/hourly/air_temperature/historical/";
    let station_id_osna = "01766";

    let temperature_hourly_file_re = Regex::new(
        r"(?P<file_name>stundenwerte_TU_(?P<station_id>[0-9]{5})_(?:akt|(?:[0-9]{8}_[0-9]{8}_hist)).zip)</a>",
    )?;

    let tmp_dir = Builder::new().prefix("historical_weather").tempdir()?;
    let response = reqwest::get(dwd_url).await?;
    let toc = response.text().await?;
    for cap in temperature_hourly_file_re
        .captures_iter(&toc)
        .filter(|c| &c["station_id"] == station_id_osna)
    {
        println!("Downloading {}", &cap["file_name"]);
        let fname = tmp_dir.path().join(&cap["file_name"]);
        let mut dest = File::create(fname)?;
        let download_url = format!("{}/{}", dwd_url, &cap["file_name"]);
        let download_response = reqwest::get(download_url).await?;
        let content = download_response.text().await?;
        copy(&mut content.as_bytes(), &mut dest)?;
        println!("DONE")
    }

    Ok(())
}
