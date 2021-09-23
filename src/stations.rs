use std::collections::HashMap;

fn build_station_map() -> HashMap<String, String> {
    let mut station_map = HashMap::new();
    station_map.insert("Osnabrück".to_owned(), "01766".to_owned());
    station_map.insert("Lingen".to_owned(), "03023".to_owned());
    station_map

}

pub fn get_station_id_by_name(station_name: &str) -> Option<String> {
    let station_map = build_station_map();

    station_map.get(station_name).map(|s| s.to_owned())
}

pub fn print_all_station_names() {
    let station_map = build_station_map();
    for station in station_map.keys() {
        println!("{}", station);
    }
}