extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct City {
    country: String,
    name: String,
    lat: String,
    lng: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Flag {
    id: i32,
    name: String,
    isoAlpha2: String,
    isoAlpha3: String,
    isoNumeric: i32,
    currency: Currency,
    flag: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Currency {}

#[derive(Serialize, Deserialize, Debug)]
struct CityFinal {
    id: i32,
    name: String,
    lat: f32,
    lng: f32,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CountryFinal {
    country: String,
    flag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Final {
    filter: Vec<CityFinal>,
    country: Vec<CountryFinal>,
}

fn main() {
    const PATH_CITY: &str = "assets/citys.json";
    let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
    file_path.push(std::env::current_dir().unwrap().as_path());
    file_path.push(PATH_CITY);
    let mut s = String::new();
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let _deserialized_city: Vec<City> = serde_json::from_str(&s).unwrap();

    const PATH_FLAG: &str = "assets/flags.json";
    let mut file_path_flag: std::path::PathBuf = std::path::PathBuf::new();
    file_path_flag.push(std::env::current_dir().unwrap().as_path());
    file_path_flag.push(PATH_FLAG);
    let mut f = String::new();
    File::open(file_path_flag.as_path())
        .unwrap()
        .read_to_string(&mut f)
        .unwrap();
    let _deserialized_flag: Vec<Flag> = serde_json::from_str(&f).unwrap();

    let mut city_final: Vec<CityFinal> = Vec::new();
    let mut i: i32 = 0;
    // let mut flag: String;
    for x in &_deserialized_city {
        i += 1;
        /*flag = "".to_string();
        for y in &_deserialized_flag {
            if y.isoAlpha2 == x.country.clone() {
                flag = y.flag.clone();
                break;
            }
        }*/
        city_final.push(CityFinal {
            id: i,
            name: x.name.clone(),
            lat: x.lat.clone().parse().unwrap(),
            lng: x.lng.clone().parse().unwrap(),
            country: x.country.clone(),
        });
    }
    let mut country_final: Vec<CountryFinal> = Vec::new();
    for x in &_deserialized_flag {
        country_final.push(CountryFinal {
            country: x.isoAlpha2.clone(),
            flag: x.flag.clone(),
        });
    }

    // final
    let filter_country: Final = Final {
        filter: city_final,
        country: country_final,
    };
    // Write new file
    let _serialized: String = serde_json::to_string(&filter_country).unwrap();
    const PATH_WRITE: &str = "assets/citys_flags.json";
    let mut file_path_write: std::path::PathBuf = std::path::PathBuf::new();
    file_path_write.push(std::env::current_dir().unwrap().as_path());
    file_path_write.push(PATH_WRITE);
    let mut buffer = File::create(file_path_write.as_path()).unwrap();
    buffer.write_all(_serialized.as_bytes()).unwrap();
    buffer.flush().unwrap();
}
