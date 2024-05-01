use super::{Serialize, Deserialize, path, fs, PathBuf};


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
pub struct PersistentG {
    pub name: String,
    pub features: Vec<Features>,
}


impl Default for PersistentG {
    fn default() -> Self {
        let path = match fs::canonicalize(super::FILE_PATH_GEOJSON) {
            Ok(path) => path,
            Err(err) => {
                log::warn!("Не удалось канонизировать файл ({}) по пути {}", err, super::FILE_PATH_GEOJSON);
                
                PathBuf::from(super::FILE_PATH_GEOJSON)
            },
        };
        let path = path.to_str()
            .expect(&format!("Не удалось перевести объект пути {:?} в строку", path));

        return if path::Path::new(path).exists() {
            let data = fs::read(path)
                .expect(&format!("Ошибка! Не удалось прочитать файл по пути {}", path));

            let data: PersistentG = serde_json::from_slice(&data)
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

