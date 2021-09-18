use anyhow::Result;
use chrono::prelude::*;

use clap::{AppSettings, Clap};
use std::fs;
use std::io;
use tempfile::Builder;

mod data_access;
mod downloader;
mod temperature_calculator;

#[derive(Clap)]
#[clap(version = "0.1", author = "Hendrik N.")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the day to query. Default value is zero which will be intepreted as today's day
    #[clap(short, long, default_value = "0")]
    day: u32,
    /// Sets the month to query. Default value is zero which will be intepreted as today's month
    #[clap(short, long, default_value = "0")]
    month: u32,
}

fn handle_cli_opt(opts: Opts) -> (u32, u32) {
    let local_now: DateTime<Local> = Local::now();
    match (opts.day, opts.month) {
        (0, 0) => (local_now.day(), local_now.month()),
        (0, x) => (local_now.day(), x),
        (x, 0) => (x, local_now.month()),
        (x, y) => (x, y),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let (target_day, target_month) = handle_cli_opt(opts);

    let station_id_osna = "01766";
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

    let (min_temp, max_temp) = temperature_calculator::get_average_temperatures(
        &measurement_vec,
        target_day,
        target_month,
    );

    println!(
        "date= {}-{} average min temperature = {}, average max temperature = {}",
        target_day, target_month, min_temp, max_temp
    );

    Ok(())
}
