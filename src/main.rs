use anyhow::Result;
use std::fs;
use std::io;
use tempfile::Builder;

mod data_access;
mod downloader;
mod temperature_calculator;

#[tokio::main]
async fn main() -> Result<()> {
    let station_id_osna = "01766";
    let target_day = 16;
    let target_month = 9;
    let tmp_dir = Builder::new().prefix("historical_weather").tempdir()?;
    let zipfile = downloader::download_zip_archive(tmp_dir.path(), station_id_osna).await?;
    let zipdir = tmp_dir.path();
    let file = fs::File::open(&zipfile)?;
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive.extract(zipdir)?;

    let paths = fs::read_dir(zipdir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let item_path = paths
        .iter()
        .find(|x| x.to_str().unwrap().contains("produkt_tu"))
        .unwrap();

    let measurement_vec = data_access::load_data(item_path)?;

    let (min_temp, max_temp) =
        temperature_calculator::get_average_temperatures(&measurement_vec, target_day, target_month);

    println!(
        "average min temperature = {}, average max temperature = {}",
        min_temp, max_temp
    );

    Ok(())
}
