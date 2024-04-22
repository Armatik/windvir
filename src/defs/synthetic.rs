use rand::Rng;
use crate::graphics;
use super::error::{self, WrongFigure};


const RED_ADJUSMENT: f32 = 0.8;
const SEGMENTS: f32 = 25.;
const DELTA_PHI: f32 = std::f32::consts::PI / SEGMENTS;
const SEGMENTS_NUM: usize = SEGMENTS as usize * 2 + 1;


pub enum SyntheticVariant {
    /// Первый аргумент `центр`. Второй аргумент `радиус`
	Circle(super::Point, f32),
    /// Первый аргумент `самая левая верхняя точка`. Второй аргумент `самая правая нижняя точка`
	Rectangle(super::Point, super::Point),
    /// Первый аргумент `первая точка` отрезка. Второй аргумент `второя точка` отрезка
	Segment(super::Point, super::Point),
    /// Рандомная фигура, содержит в себе вектор вершин
    Polygon(Vec<graphics::Vertex>),
}


#[derive(PartialEq)]
pub enum SimplySyntheticVariant {
    Circle,
    Rectangle,
    Segment,
    Polygon,
}


pub trait SyntheticData {
    fn get_data(&self) -> SyntheticVariant;
    fn get_data_simply(&self) -> SimplySyntheticVariant;
	fn is_value_default(&self) -> bool;
	fn set_value(&mut self, data: SyntheticVariant);
    /// Возвращается ошибка в случае если у структуры присутсвует != 2 точки
    fn set_points(&mut self, points: Vec<super::Point>) -> error::Result<()>;
    fn get_rgb(&self) -> (f32, f32, f32);
    #[allow(dead_code)]
    fn set_rgb(&mut self, r: f32, g: f32, b: f32);
    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>);
    fn get_primitive(&self) -> glium::index::PrimitiveType;
    /// Возвращается ошибка в случае если невозможно изменить примитив
    fn change_primitive(&mut self) -> error::Result<()>;
}


pub struct Circle {
    center: super::Point,
	radius: f32,
    is_fill: bool,
    rgb: (f32, f32, f32),
}


impl Circle {
    pub fn new(rgb: Option<(f32, f32, f32)>) -> Self {
        let rgb = match rgb {
            Some(rgb) => rgb,
            None => {
                let mut rng = rand::thread_rng();

                (rng.gen::<f32>() * RED_ADJUSMENT, rng.gen::<f32>(), rng.gen::<f32>())
            },
        };

        Self {
            center: super::Point::default(),
            radius: f32::default(),
            is_fill: true,
            rgb,
        }
    }
}


impl SyntheticData for Circle {
    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Circle(self.center.clone(), self.radius)
    }

    fn get_data_simply(&self) -> SimplySyntheticVariant {
        SimplySyntheticVariant::Circle
    }

	fn is_value_default(&self) -> bool {
		self.radius == f32::default() || self.center == super::Point::default()
	}

	fn set_value(&mut self, data: SyntheticVariant) {
		if let SyntheticVariant::Circle(center, radius) = data {
            self.center = center;
            self.radius = radius;
		} else {
			log::warn!("Данные для созданной окружности могут быть заданы не верно!");
        }
	}

    fn set_points(&mut self, _points: Vec<super::Point>) -> error::Result<()> {
        Err(error::WrongFigure)
    }

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {
        let mut phi = 0.;
        let mut vertices = Vec::<graphics::Vertex>::with_capacity(SEGMENTS_NUM);
        let mut indices = Vec::<u16>::with_capacity(SEGMENTS_NUM * 3 - 3);
        let x = self.center.x;
        let y = self.center.y;
        let r = self.radius;
        
        vertices.push(graphics::Vertex { position: [x, y] });

        while phi < 2. * std::f32::consts::PI + DELTA_PHI {
            let x = x + r * f32::cos(phi);
            let y = y + r * f32::sin(phi);

            vertices.push(graphics::Vertex { position: [x, y] });

            phi += DELTA_PHI;
        }

        if self.is_fill {
            for index in 1..SEGMENTS_NUM as u16 {
                indices.append(&mut vec![0, index, index + 1]);
            }
        } else {
            for index in 1..SEGMENTS_NUM as u16 - 1 {
                indices.append(&mut vec![index, index + 1]);
            }
        }

        (vertices, Some(indices))
    }

    fn get_primitive(&self) -> glium::index::PrimitiveType {
        return if self.is_fill {
            glium::index::PrimitiveType::TrianglesList
        } else {
            glium::index::PrimitiveType::LineLoop
        }
    }

    fn change_primitive(&mut self) -> error::Result<()> {
        self.is_fill = !self.is_fill;

        Ok(())
    }
}


pub struct Rectangle {
    left_up_point: super::Point,
    right_down_point: super::Point,
    is_fill: bool,
    rgb: (f32, f32, f32),
}


impl Rectangle {
    pub fn new(rgb: Option<(f32, f32, f32)>) -> Self {
        let rgb = match rgb {
            Some(rgb) => rgb,
            None => {
                let mut rng = rand::thread_rng();

                (rng.gen::<f32>() * RED_ADJUSMENT, rng.gen::<f32>(), rng.gen::<f32>())
            },         
        };
        
        Self {
            left_up_point: super::Point::default(),
            right_down_point: super::Point::default(),
            is_fill: true,
            rgb,
        }
    }
}


impl SyntheticData for Rectangle {
    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Rectangle(self.left_up_point.clone(), self.right_down_point.clone())
    }

    fn get_data_simply(&self) -> SimplySyntheticVariant {
        SimplySyntheticVariant::Rectangle
    }

	fn is_value_default(&self) -> bool {
		self.left_up_point == super::Point::default() || self.right_down_point == super::Point::default()
	}

	fn set_value(&mut self, data: SyntheticVariant) {
        if let SyntheticVariant::Rectangle(left_up_point, right_down_point) = data {
            self.left_up_point = left_up_point;
			self.right_down_point = right_down_point;
        } else {
            log::error!("Данные для созданного прямоугольника могут быть заданы не верно!");
        }
	}

    fn set_points(&mut self, points: Vec<super::Point>) -> error::Result<()> {
        if points.len() != 2 {
            log::error!("Для прямоуольника задано кол-во точек != 2!");

            return Err(error::WrongFigure);
        }
        
        self.left_up_point = points[0].clone();
        self.right_down_point = points[1].clone();
        
        Ok(())
    }

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {
        let lu_x = self.left_up_point.x;
        let lu_y = self.left_up_point.y;
        let rd_x = self.right_down_point.x;
        let rd_y = self.right_down_point.y;

        let vertices = vec![graphics::Vertex { position: [lu_x, lu_y] },
            graphics::Vertex { position: [rd_x, lu_y] },
            graphics::Vertex { position: [rd_x, rd_y] },
            graphics::Vertex { position: [lu_x, rd_y] }];

        return if self.is_fill {
            (vertices, Some(vec![0, 1, 2, 0, 3, 2]))
        } else {
            (vertices, Some(vec![0, 1, 1, 2, 2, 3]))
        }
    }

    fn get_primitive(&self) -> glium::index::PrimitiveType {
        return if self.is_fill {
            glium::index::PrimitiveType::TrianglesList
        } else {
            glium::index::PrimitiveType::LineLoop
        }
    }

    fn change_primitive(&mut self) -> error::Result<()> {
        self.is_fill = !self.is_fill;

        Ok(())
    }
}


pub struct Segment {
	p0: super::Point,
	p1: super::Point,
    rgb: (f32, f32, f32),
}


impl Segment {
    pub fn new(rgb: Option<(f32, f32, f32)>) -> Self {
        let rgb = match rgb {
            Some(rgb) => rgb,
            None => {
                let mut rng = rand::thread_rng();

                (rng.gen::<f32>() * RED_ADJUSMENT, rng.gen::<f32>(), rng.gen::<f32>())
            },
        };

        Self {
            p0: super::Point::default(),
            p1: super::Point::default(),
            rgb,
        }
    }
}


impl SyntheticData for Segment {
    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Segment(self.p0.clone(), self.p1.clone())
    }

    fn get_data_simply(&self) -> SimplySyntheticVariant {
        SimplySyntheticVariant::Segment
    }

	fn is_value_default(&self) -> bool {
		self.p0 == super::Point::default() || self.p1 == super::Point::default()
	}

	fn set_value(&mut self, data: SyntheticVariant) {
		if let SyntheticVariant::Segment(p0, p1) = data {
            self.p0 = p0;
            self.p1 = p1;
		} else {
			log::error!("Данные для отрезка могут быть заданы не верно!");
        }
	}

    fn set_points(&mut self, points: Vec<super::Point>) -> error::Result<()> {
        if points.len() != 2 {
            log::error!("Для отрезка задано кол-во точек != 2!");

            return Err(error::WrongFigure);
        }
        self.p0 = points[0].clone();
        self.p1 = points[1].clone();
        
        Ok(())
    }

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {
        let p0_x = self.p0.x;
        let p0_y = self.p0.y;
        let p1_x = self.p1.x;
        let p1_y = self.p1.y;

        (vec![graphics::Vertex { position: [p0_x, p0_y] }, graphics::Vertex { position: [p1_x, p1_y] }], None)
    }

    fn get_primitive(&self) -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::LineLoop
    }

    fn change_primitive(&mut self) -> error::Result<()> {
        Err(WrongFigure)
    }
}


pub struct Polygon {
    points: Vec<graphics::Vertex>,
    is_fill: bool,
    rgb: (f32, f32, f32),
}


impl Polygon {
    pub fn new(rgb: Option<(f32, f32, f32)>) -> Self {
        let rgb = match rgb {
            Some(rgb) => rgb,
            None => {
                let mut rng = rand::thread_rng();

                (rng.gen::<f32>() * RED_ADJUSMENT, rng.gen::<f32>(), rng.gen::<f32>())
            },
        };

        Self {
            points: Vec::new(),
            is_fill: true,
            rgb,
        } 
    }
}


impl SyntheticData for Polygon {
    fn change_primitive(&mut self) -> error::Result<()> {
        self.is_fill = !self.is_fill;

        Ok(())
    }

    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Polygon(self.points.clone())
    }

    fn get_data_simply(&self) -> SimplySyntheticVariant {
        SimplySyntheticVariant::Polygon
    }

    fn get_primitive(&self) -> glium::index::PrimitiveType {
        return if self.is_fill {
            glium::index::PrimitiveType::TrianglesList
        } else {
            glium::index::PrimitiveType::LineLoop
        }
    }

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {        
        let mut indices = Vec::<u16>::new();
        
        if self.is_fill {
            let mut points = Vec::<f32>::with_capacity(self.points.len() * 2);
            
            for point in &self.points {
                points.append(&mut vec![point.position[0], point.position[1]]);
            }

            let idx = earcutr::earcut(&points, &[], 2).expect("Ошибка! Не удалось триангулировать многоугольник!");

            indices = idx.iter().map(|x| *x as u16).collect::<Vec<u16>>();
        } else {
            for i in 0..self.points.len() - 1 {
                indices.append(&mut vec![i as u16, i as u16 + 1]);
            }
        }

        (self.points.clone(), Some(indices))
    }

    fn is_value_default(&self) -> bool {
        self.points == Vec::new()
    }

    fn set_points(&mut self, points: Vec<super::Point>) -> error::Result<()> {
        self.points = points.iter().map(|x| graphics::Vertex { position: [x.x, x.y] }).collect::<Vec<graphics::Vertex>>();

        Ok(())
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn set_value(&mut self, data: SyntheticVariant) {
        if let SyntheticVariant::Polygon(points) = data {
            self.points = points;
        } else {
			log::error!("Данные для многоугольника могут быть заданы не верно!");
        }
    }
}
