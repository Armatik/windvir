use super::{Serialize, Deserialize, path, fs, PathBuf};


#[derive(Debug, Serialize, Deserialize)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub is_fill: bool,
    pub rgb: [f32; 3],
}


impl Default for Circle {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            radius: 0.,
            is_fill: true,
            rgb: [0., 0., 0.],
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Rectangle {
    pub left_up_angle_x: f32,
    pub left_up_angle_y: f32,
    pub right_down_angle_x: f32,
    pub right_down_angle_y: f32,
    pub is_fill: bool,
    pub rgb: [f32; 3],
}


impl Default for Rectangle {
    fn default() -> Self {
        Self {
            left_up_angle_x: 0.,
            left_up_angle_y: 0.,
            right_down_angle_x: 0.,
            right_down_angle_y: 0.,
            is_fill: true,
            rgb: [0., 0., 0.],
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub p0_x: f32,
    pub p0_y: f32,
    pub p1_x: f32,
    pub p1_y: f32,
    pub rgb: [f32; 3],
}


impl Default for Line {
    fn default() -> Self {
        Self {
            p0_x: 0.,
            p0_y: 0.,
            p1_x: 0.,
            p1_y: 0.,
            rgb: [0., 0., 0.],
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Vec<f32>>,
    pub is_fill: bool,
    pub rgb: [f32; 3],
}


impl Default for Polygon {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            is_fill: true,
            rgb: [0., 0., 0.],
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentF {
    pub circles: Option<Vec<Circle>>,
    pub rectangles: Option<Vec<Rectangle>>,
    pub lines: Option<Vec<Line>>,
    pub polygons: Option<Vec<Polygon>>,
}


impl Default for PersistentF {
    fn default() -> Self {
        let path = match fs::canonicalize(super::FILE_PATH_FIGURES) {
            Ok(path) => path,
            Err(err) => {
                log::warn!("Не удалось канонизировать файл ({}) по пути {}", err, super::FILE_PATH_FIGURES);
                
                PathBuf::from(super::FILE_PATH_FIGURES)
            },
        };
        let path = path.to_str()
            .expect(&format!("Не удалось перевести объект пути {:?} в строку", path));

        return if path::Path::new(path).exists() {
            let data = fs::read(path)
                .expect(&format!("Ошибка! Не удалось прочитать файл по пути {}", path));

            let data: PersistentF = serde_json::from_slice(&data)
                .expect(&format!("Ошибка! Не удалось прочитать GEOJSON по пути {}", path));

            data
        } else {
            Self {
                circles: None,
                rectangles: None,
                lines: None,
                polygons: None,
            }
        }
    }
}
