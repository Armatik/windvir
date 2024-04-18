use rand::Rng;
use crate::graphics;


const RED_ADJUSMENT: f32 = 0.8;
pub const AIM_INDEX: usize = 0;


pub enum SyntheticVariant {
    /// Первый аргумент `центр`. Второй аргумент `радиус`
	Circle((f64, f64), f64),
    /// Первый аргумент `самая левая верхняя точка`. Второй аргумент `самая правая нижняя точка`
	Rectangle((f64, f64), (f64, f64)),
    /// Первый аргумент `первая точка` отрезка. Второй аргумент `второя точка` отрезка
	Segment((f64, f64), (f64, f64)),
}


pub trait SyntheticData {
    fn get_data(&self) -> SyntheticVariant;
	fn is_value_default(&self) -> bool;
	fn set_value(&mut self, data: SyntheticVariant);
    fn get_rgb(&self) -> (f32, f32, f32);
    fn set_rgb(&mut self, r: f32, g: f32, b: f32);
    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>);
}


pub struct Circle {
    center: (f64, f64),
	radius: f64,
    rgb: (f32, f32, f32),
}


impl Default for Circle {
    fn default() -> Self {
        let def = f64::default();
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>() * RED_ADJUSMENT;
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        
        Self {
            center: (def, def),
            radius: def,
            rgb: (r, g, b),
        }
    }
}


impl SyntheticData for Circle {
    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Circle(self.center, self.radius)
    }

	fn is_value_default(&self) -> bool {
		self.radius == f64::default() || self.center == (f64::default(), f64::default())
	}

	fn set_value(&mut self, data: SyntheticVariant) {
		if let SyntheticVariant::Circle(center, radius) = data {
            self.center = center;
            self.radius = radius;
		} else {
			log::warn!("Данные для созданной окружности могут быть заданы не верно!");
        }
	}

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {
        const SEGMENTS: f64 = 25.;
        const DELTA_PHI: f64 = std::f64::consts::PI / SEGMENTS;
        const SEGMENTS_NUM: usize = SEGMENTS as usize * 2 + 1;
        let mut phi = 0.;
        let mut vertices = Vec::<graphics::Vertex>::with_capacity(SEGMENTS_NUM);
        let mut indices = Vec::<u16>::with_capacity(SEGMENTS_NUM - 1);
        let (x, y) = self.center;
        let r = self.radius;
        
        vertices.push(graphics::Vertex { position: [x, y] });

        while phi < 2. * std::f64::consts::PI + DELTA_PHI {
            let x = x + r * f64::cos(phi);
            let y = y + r * f64::sin(phi);

            vertices.push(graphics::Vertex { position: [x, y] });

            phi += DELTA_PHI;
        }

        for index in 1..SEGMENTS_NUM as u16 {
            indices.append(&mut vec![0, index, index + 1]);
        }

        (vertices, Some(indices))
    }
}


pub struct Rectangle {
    left_up_point: (f64, f64),
    right_down_point: (f64, f64),
    rgb: (f32, f32, f32),
}


impl Default for Rectangle {
    fn default() -> Self {
        let def = (f64::default(), f64::default());
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>() * RED_ADJUSMENT;
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();

        Self {
            left_up_point: def,
            right_down_point: def,
            rgb: (r, g, b),
        }
    }
}


impl SyntheticData for Rectangle {
    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Rectangle(self.left_up_point, self.right_down_point)
    }

	fn is_value_default(&self) -> bool {
		self.left_up_point == (f64::default(), f64::default()) || self.right_down_point == (f64::default(), f64::default())
	}

	fn set_value(&mut self, data: SyntheticVariant) {
        if let SyntheticVariant::Rectangle(left_up_point, right_down_point) = data {
            self.left_up_point = left_up_point;
			self.right_down_point = right_down_point;
        } else {
            log::warn!("Данные для созданного прямоугольника могут быть заданы не верно!");
        }
	}

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {
        let (lu_x, lu_y) = self.left_up_point;
        let (rd_x, rd_y) = self.right_down_point;

        (vec![graphics::Vertex { position: [lu_x, lu_y] },
            graphics::Vertex { position: [rd_x, lu_y] },
            graphics::Vertex { position: [rd_x, rd_y] },
            graphics::Vertex { position: [lu_x, rd_y] }
            ], Some(vec![0, 1, 2, 0, 3, 2]))
    }
}


pub struct Segment {
	p0: (f64, f64),
	p1: (f64, f64),
    rgb: (f32, f32, f32),
}


impl Default for Segment {
    fn default() -> Self {
        let def = (f64::default(), f64::default());
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>() * RED_ADJUSMENT;
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();

        Self {
            p0: def,
            p1: def,
            rgb: (r, g, b),
        }
    }
}


impl SyntheticData for Segment {
    fn get_data(&self) -> SyntheticVariant {
        SyntheticVariant::Segment(self.p0, self.p1)
    }

	fn is_value_default(&self) -> bool {
		self.p0 == (f64::default(), f64::default()) || self.p1 == (f64::default(), f64::default())
	}

	fn set_value(&mut self, data: SyntheticVariant) {
		if let SyntheticVariant::Segment(p0, p1) = data {
            self.p0 = p0;
            self.p1 = p1;
		} else {
			log::warn!("Данные для отрезка могут быть заданы не верно!");
        }
	}

    fn get_rgb(&self) -> (f32, f32, f32) {
        self.rgb
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        self.rgb = (r, g, b);
    }

    fn get_vertices_and_indices(&self) -> (Vec<graphics::Vertex>, Option<Vec<u16>>) {
        let (p0_x, p0_y) = self.p0;
        let (p1_x, p1_y) = self.p1;

        (vec![graphics::Vertex { position: [p0_x, p0_y] }, graphics::Vertex { position: [p1_x, p1_y] }], None)
    }
}
