use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use std::fs;
use std::io;
use tempfile::Builder;

mod data_access;
mod downloader;

#[tokio::main]
async fn main() -> Result<()> {
    let station_id_osna = "01766";
    let tmp_dir = Builder::new().prefix("historical_weather").tempdir()?;
    let zipfile = downloader::download_zip_archive(tmp_dir.path(), station_id_osna).await?;
    let zipdir = tmp_dir.path();
    let file = fs::File::open(&zipfile)?;
    let mut archive = zip::ZipArchive::new(file).unwrap();
    println!("DONE");
    archive.extract(zipdir)?;

    let paths = fs::read_dir(zipdir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let item_path = paths
        .iter()
        .find(|x| x.to_str().unwrap().contains("produkt_tu"))
        .unwrap();

    println!("{}", item_path.to_str().unwrap());

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(item_path)?;

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

        measurement_vec.push(data_access::TemperatureMeasurement::new(
            NaiveDate::from_ymd(year, month, day).and_hms(hour, 0, 0),
            record.get(3).unwrap().trim().parse::<f32>()?,
        ));
    }

    Ok(())
}
