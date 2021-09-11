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
    let split = toc
        .split('\n')
        .filter(|x| x.contains(&format!("stundenwerte_TU_{}", station_id_osna)));
    for s in split {
        for cap in temperature_hourly_file_re.captures_iter(s) {
            println!("{:?}", &cap["file_name"]);
        }
    }

    // let mut dest = {
    //     let fname = response
    //         .url()
    //         .path_segments()
    //         .and_then(|segments| segments.last())
    //         .and_then(|name| if name.is_empty() { None } else { Some(name) })
    //         .unwrap_or("tmp.bin");

    //     println!("file to download: '{}'", fname);
    //     let fname = tmp_dir.path().join(fname);
    //     println!("will be located under: '{:?}'", fname);
    //     File::create(fname)?
    // };
    // let content = response.text().await?;
    // copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
