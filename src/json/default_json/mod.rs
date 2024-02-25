use super::{Serialize, Deserialize, path, fs, PathBuf};


const FILE_PATH: &str = "./default_settings.json";


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MapOffset {
    pub x: f32,
    pub y: f32,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct BackgroundColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Movement {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub theta: f32,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PersistentJ {
    pub resolution: Resolution,
    pub map_offset: MapOffset,
    pub background_color: BackgroundColor,
    pub movement: Movement,
    pub scale: f32,
    pub theta: f32,
    pub multisampling: u16,
    pub depth_buffer: u8,
}


impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
        }
    }
}


impl Default for MapOffset {
    fn default() -> Self {
        Self {
            x: -45.395,
            y: -52.293,
        }
    }
}


impl Default for BackgroundColor {
    fn default() -> Self {
        Self {
            r: 0.8,
            g: 0.98,
            b: 0.988,
            a: 1.,
        }
    }
}


impl Default for Movement {
    fn default() -> Self {
        Self {
            x: 0.0001,
            y: 0.00007,
            scale: 7.,
            theta: 0.04,
        }
    }
}


impl Default for PersistentJ {
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

            let data: PersistentJ = serde_json::from_slice(&data)
                .expect(&format!("Ошибка! Не удалось прочитать GEOJSON по пути {}", path));

            Self {
                resolution: data.resolution,
                map_offset: data.map_offset,
                background_color: data.background_color,
                movement: data.movement,
                scale: data.scale,
                theta: data.theta,
                multisampling: data.multisampling,
                depth_buffer: data.depth_buffer,
            }
        } else {
            Self {
                scale: 180.,
                theta: 0.,
                multisampling: 8,
                depth_buffer: 24,
                ..Default::default()
            }
        }
    }
}

