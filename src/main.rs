use anyhow::Result;
use std::fs;
use std::io;
use tempfile::Builder;

mod downloader;
mod data_access;

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
        measurement_vec.push(data_access::TemperatureMeasurement::new(record.get(1).unwrap().to_string(),
        record.get(3).unwrap().trim().parse::<f32>()?));
    }

    Ok(())
}
