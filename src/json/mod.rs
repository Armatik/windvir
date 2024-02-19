use serde::{Serialize, Deserialize};
use std::{
    path,
    fs,
};


const FILE_PATH: &str = "./data/Здания элеваторского района (big data).geojson";


#[derive(Debug, Serialize, Deserialize)]
struct Properties {
    object: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    tupe: String,
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Features {
    tupe: String,
    properties: Properties,
    pub geometry: Geometry,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Persistent {
    pub name: String,
    pub features: Vec<Features>,
}


impl Default for Persistent {
    fn default() -> Self {
        let path = fs::canonicalize(FILE_PATH)
            .expect(&format!("Ошибка! Не удалось канонизировать относительный путь {}", FILE_PATH));
        let path = path.to_str()
            .expect(&format!("Ошибка! Не удалось перевести объект {:?} в строку", path));

        return if path::Path::new(path).exists() {
            let data = fs::read(path)
                .expect(&format!("Ошибка! Не удалось прочитать файл по пути {}", path));

            let data: Persistent = serde_json::from_slice(&data)
                .expect(&format!("Ошибка! Не удалось прочитать GEOJSON по пути {}", path));

            Self {
                name: data.name,
                features: data.features,
            }
        } else {
            Self {
                name: String::from("Здания элеваторского района"),
                features: Vec::new(),
            }
        }
    }
}

