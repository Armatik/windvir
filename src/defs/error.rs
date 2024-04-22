use std::{error, fmt};


pub type Result<T> = std::result::Result<T, WrongFigure>;


pub struct WrongFigure;


impl error::Error for WrongFigure {}


impl fmt::Debug for WrongFigure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Была использована неверная фигура!")
    }
}


impl fmt::Display for WrongFigure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Была использована неверная фигура")
    }
}

