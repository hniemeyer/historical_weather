use anyhow::Result;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::Builder;

async fn download_zip_archive(download_path: &Path, station_id: &str) -> Result<PathBuf> {
    let dwd_url = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/hourly/air_temperature/historical/";
    let temperature_hourly_file_re = Regex::new(
        r"(?P<file_name>stundenwerte_TU_(?P<station_id>[0-9]{5})_(?:akt|(?:[0-9]{8}_[0-9]{8}_hist)).zip)</a>",
    )?;

    
    let response = reqwest::get(dwd_url).await?;
    let toc = response.text().await?;
    let zip = temperature_hourly_file_re.captures_iter(&toc).filter(|c| &c["station_id"] == station_id).take(1).collect::<Vec<_>>();
    let fname = download_path.join(&zip[0]["file_name"]);
    let mut dest = File::create(&fname)?;
    let download_url = format!("{}/{}", dwd_url, &zip[0]["file_name"]);
    let download_response = reqwest::get(download_url).await?;
    let content = download_response.bytes().await?;
    dest.write_all(&content)?;
    Ok(fname)

}

#[tokio::main]
async fn main() -> Result<()> {
    let station_id_osna = "01766";
    let tmp_dir = Builder::new().prefix("historical_weather").tempdir()?;
    let zipfile = download_zip_archive(tmp_dir.path(), station_id_osna).await?;
    let zipdir = tmp_dir.path();
    let file = fs::File::open(&zipfile)?;
    let mut archive = zip::ZipArchive::new(file).unwrap();
    println!("DONE");
    archive.extract(zipdir)?;
    

    let paths = fs::read_dir(zipdir)?;

    for path in paths {
        println!("Name: {}", path?.path().display())
    }

    Ok(())
}
