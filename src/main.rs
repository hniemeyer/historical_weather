use anyhow::Result;
use std::fs;
use tempfile::Builder;

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

    let paths = fs::read_dir(zipdir)?;

    for path in paths {
        let path_buf = path.unwrap().path().to_owned();
        let path_str = path_buf.to_str().unwrap();
        if path_str.contains("produkt_tu") {
        println!("{}", path_str)
        }
    }

    Ok(())
}
