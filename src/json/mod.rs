use serde::{Serialize, Deserialize};
use std::{path::{self, PathBuf}, fs};


const FILE_PATH_DEFAULT: &str = "./default_settings.json";
const FILE_PATH_GEOJSON: &str = "./data/Здания элеваторского района (big data).geojson";
const FILE_PATH_FIGURES: &str = "./data/figures.json";


pub mod geojson;
pub mod default_json;
pub mod figures;
