use super::{Serialize, Deserialize, path, fs, PathBuf};


const FILE_PATH: &str = "./data/Здания элеваторского района (big data).geojson";


#[derive(Debug, Serialize, Deserialize)]
struct Properties {
    object: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    tupe: String,
    pub coordinates: Vec<Vec<Vec<Vec<f32>>>>,
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
        let path = match fs::canonicalize(FILE_PATH) {
            Ok(path) => path,
            Err(err) => {
                log::warn!("Не удалось канонизировать файл ({}) по пути {}", err, FILE_PATH);
                
                PathBuf::from(FILE_PATH)
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

