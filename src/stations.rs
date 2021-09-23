use std::collections::HashMap;

pub fn get_station_id_by_name(station_name: &str) -> Option<String> {
    let mut station_map = HashMap::new();
    station_map.insert("Osnabrück".to_owned(), "01766".to_owned());

    station_map.get(station_name).map(|s|s.to_owned())


}