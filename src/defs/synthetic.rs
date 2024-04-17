const AIM_RADIUS: f64 = 0.00005;
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
    /// Возвращается `None` в случае, если объект не является окружностью
    fn move_aim(&mut self, delta_x: f64, delta_y: f64) -> Option<()>;
}


#[derive(Default)]
pub struct Circle {
    center: (f64, f64),
	radius: f64,
}


impl Circle {
	pub fn new_aim(x: f64, y: f64) -> Self {
		Self {
			center: (x, y),
			radius: AIM_RADIUS,
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

    fn move_aim(&mut self, delta_x: f64, delta_y: f64) -> Option<()> {
        self.center.0 += delta_x;
        self.center.1 += delta_y;

        Some(())
    }
}


#[derive(Default)]
pub struct Rectangle {
    left_up_point: (f64, f64),
    right_down_point: (f64, f64),
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

    fn move_aim(&mut self, _delta_x: f64, _delta_y: f64) -> Option<()> {
        None
    }
}


#[derive(Default)]
pub struct Segment {
	p0: (f64, f64),
	p1: (f64, f64),
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

    fn move_aim(&mut self, _delta_x: f64, _delta_y: f64) -> Option<()> {
        None
    }
}
