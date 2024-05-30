use crate::{App, graphics, etc, defs::synthetic, ffi::{self, ReprRust}, collisions};
use std::alloc::{alloc, Layout};


const W: f64 = 0.5;


pub struct FigureIndices<T> where T: glium::index::Index {
    pub buildings_indices_line: glium::IndexBuffer<T>,
    pub buildings_indices_triangulate: glium::IndexBuffer<T>,
    pub field_indices: glium::IndexBuffer<T>,
}


impl<T> FigureIndices<T> where T: glium::index::Index {
    pub fn new(
        buildings_indices_line: glium::IndexBuffer<T>,
        buildings_indices_triangulate: glium::IndexBuffer<T>,
        field_indices: glium::IndexBuffer<T>,
    ) -> Self {
        Self {
            buildings_indices_line,
            buildings_indices_triangulate,
            field_indices,
        }
    }
}


pub struct Positions {
    pub change_positions: glium::VertexBuffer<graphics::Vertex>,
    pub field_positions: glium::VertexBuffer<graphics::ShaderVertex>,
    pub default_positions: glium::VertexBuffer<graphics::Vertex>,
}


impl Positions {
    pub fn new(
        change_positions: glium::VertexBuffer<graphics::Vertex>,
        field_positions: glium::VertexBuffer<graphics::ShaderVertex>,
        default_positions: glium::VertexBuffer<graphics::Vertex>,
    ) -> Self {
        Self {
            change_positions,
            field_positions,
            default_positions,
        }
    }
}


impl App {
    fn get_buildings_vertices(&self) -> Vec<graphics::Vertex> {
        let mut shape = Vec::<graphics::Vertex>::with_capacity(self.buildings.len());

        for building in &self.buildings {
            for side in &building.sides {
                shape.push(graphics::Vertex { position: etc::vec_to_arr::<f32, 3>(
                    vec![side.position.x as f32, side.position.y as f32, 0.]
                ) });
            }

            for side in &building.sides {
                shape.push(graphics::Vertex { position: etc::vec_to_arr::<f32, 3>(
                    vec![side.position.x as f32, side.position.y as f32, 0.0005]
                ) });
            }
        }

        shape
    }

    fn get_default_buildings_vertices(default_buildings: &Vec<super::Building>) -> Vec<graphics::Vertex> {
        let mut shape = Vec::<graphics::Vertex>::with_capacity(default_buildings.len());

        for building in default_buildings {
            for side in &building.sides {
                shape.push(graphics::Vertex { position: etc::vec_to_arr::<f32, 3>(
                    vec![side.position.x as f32, side.position.y as f32, 0.]
                ) });
            }
        }

        shape
    }

    pub fn init_positions(&self, display: &glium::Display, default_buildings: &Vec<super::Building>) -> Result<Positions, Box<dyn std::error::Error>> {
        let shape = self.get_buildings_vertices();
        let building_vertices = glium::VertexBuffer::new(display, &shape)?;
        let default_shape = Self::get_default_buildings_vertices(default_buildings);
        let default_building_vertices = glium::VertexBuffer::new(display, &default_shape)?;

        let field_positions = self.init_field(self.rainbow_field, display)?;

        Ok(Positions::new(building_vertices, field_positions, default_building_vertices))
    }

    pub fn set_positions(&self, display: &glium::Display, positions: &mut Positions) -> Result<(), Box<dyn std::error::Error>> {
        let shape = self.get_buildings_vertices();
        positions.change_positions = glium::VertexBuffer::new(display, &shape)?;
        let default_buildings = Self::get_default_buildings_vertices(
            &self.buildings,
        );
        positions.default_positions = glium::VertexBuffer::new(
            display, &default_buildings,
        )?;


        Ok(())
    }

    fn init_field(
        &self,
        is_color_rainbow: bool,
        display: &glium::Display,
    ) -> Result<glium::VertexBuffer<graphics::ShaderVertex>, glium::vertex::BufferCreationError> {
        const CORRECTION_FACTOR: f32 = 1000.;
        let field_size = self.p_j.reverse_field_size; 
        let default_width = self.p_j.resolution.width as f32 / CORRECTION_FACTOR;
        let default_height = self.p_j.resolution.height as f32 / CORRECTION_FACTOR;
        let rgb = if is_color_rainbow {
            [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [1., 1., 0.]]
        } else {
            let default_color = self.p_j.background_color;

            [[default_color.r, default_color.g, default_color.b]; 4]
        };

        glium::VertexBuffer::new(display, &vec![
            graphics::ShaderVertex {
                position: [-1. / field_size * default_width, 1. / field_size * default_height, 0.],
                color: rgb[0],
            },
            graphics::ShaderVertex {
                position: [1. / field_size * default_width, 1. / field_size * default_height, 0.],
                color: rgb[1],
            },
            graphics::ShaderVertex {
                position: [1. / field_size * default_width, -1. / field_size * default_height, 0.],
                color: rgb[2],
            },
            graphics::ShaderVertex {
                position: [-1. / field_size * default_width, -1. / field_size * default_height, 0.],
                color: rgb[3],
            },
        ])
    }

    pub fn init_indices(&self, display: &glium::Display, default_buildings: Vec<super::Building>) -> Result<FigureIndices<u16>, glium::index::BufferCreationError> {
        let indices_line = graphics::get_line_indices(&default_buildings);
        let indices_triangulate = graphics::get_triangulation_indices(&self.buildings);

        let indices_line = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &indices_line,
        )?;
        let indices_triangulate = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices_triangulate,
        )?;
        let indices_field = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &vec![0_u16, 1, 2, 0, 2, 3],
        )?;
        let indices = FigureIndices::new(indices_line, indices_triangulate, indices_field);

        Ok(indices)
    }

    pub fn set_indices(&self, display: &glium::Display, indices: &mut FigureIndices<u16>) -> Result<(), Box<dyn std::error::Error>> {
        indices.buildings_indices_line = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &graphics::get_line_indices(&self.buildings),
        )?;
        indices.buildings_indices_triangulate = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &graphics::get_triangulation_indices(&self.buildings),
        )?;
        
        Ok(())
    }

    pub fn is_start_polygon(&self) -> bool {
        self.synthetic_datas_points.len() == 0
    }

    pub fn is_polygon(&self) -> bool {
        return match self.synthetic_data.back() {
            Some(data) => data.get_data_simply() == synthetic::SimplySyntheticVariant::Polygon,
            None => false,
        }
    }

    pub fn spawn_point(&mut self, is_end_of_polygon: bool) {
        if self.synthetic_datas_points.get(0).is_none() {
            self.synthetic_datas_points.push(super::Point::new(self.aim.x as f64, self.aim.y as f64));

            log::info!("Первая точка была успешно отмечена!");
        } else {
            match self.synthetic_data.back().unwrap().get_data_simply() {
                synthetic::SimplySyntheticVariant::Rectangle | synthetic::SimplySyntheticVariant::Segment => {
                    self.synthetic_datas_points.push(super::Point::new(self.aim.x as f64, self.aim.y as f64));
                    self.synthetic_data.back_mut().unwrap().set_points(self.synthetic_datas_points.clone())
                        .expect("Произошла ошибка! Данные точки начали задаваться для окружности!");
                },
                synthetic::SimplySyntheticVariant::Polygon => if is_end_of_polygon {
                        self.synthetic_data.back_mut().unwrap().set_points(self.synthetic_datas_points.clone())
                            .expect("Произошла ошибка! Данные точки начали задаваться для окружности!");
                    } else {
                        self.synthetic_datas_points.push(super::Point::new(self.aim.x as f64, self.aim.y as f64));

                        log::info!("Точка для многоугольника была успешно задана!");
                        return;
                },
                _ => {},
            }
            
            self.synthetic_datas_points = Vec::new();
            log::info!("Фигура была успешно задана!");
        }
    }

    pub fn spawn_circle(&mut self, value: f64) {
        if let Some(figure) = self.synthetic_data.back() {
            if figure.is_value_default() {
                let size = self.p_j.aim.aim_adjusment as f64 * value;
                self.synthetic_data.back_mut().unwrap()
                    .set_value(synthetic::SyntheticVariant::Circle(super::Point::new(self.aim.x as f64, self.aim.y as f64), size));

                log::info!("Окружность размером {size} была успешно задана!");
            }
        }
    }

    pub fn define_figure<'a, F>(&'a mut self, figure: F, log_info: &'a str) where F: synthetic::SyntheticData + 'static {
        log::info!("{log_info}");
        
        self.synthetic_data.push_back(Box::new(figure));
    }

    pub fn push_into_convex(&mut self, is_non_convex_hull: bool) {
        let (choosed_vec, color) = if is_non_convex_hull {
            (&mut self.non_choosed_buildings, graphics::SELECTED_NON_CONVEX_BUILDING_COLOR)
        } else {
            (&mut self.choosed_buildings, graphics::SELECTED_CONVEX_BUILDING_COLOR)
        };

        for (index, building) in self.buildings.iter().enumerate() {
            if collisions::test_if_point_inside_building(
                &super::PositionVector::new(self.aim.x, self.aim.y),
                &building,
            ) {
                let mut need_remove: Option<usize> = None;

                for (vec_index, (_, building_index)) in 
                choosed_vec.iter().enumerate() {
                    if index == *building_index {
                        need_remove = Some(vec_index);
                    }
                }

                if let Some(index) = need_remove {
                    choosed_vec.remove(index);
                    break;
                }

                let mut points = Vec::<Vec<f64>>::with_capacity(
                    building.sides.len()
                );
                
                for point in &building.sides {
                    points.push(vec![point.position.x, point.position.y]);
                }

                choosed_vec.push(
                    (synthetic::Polygon::init(
                        points,
                        true,
                        color,
                    ),
                    index),
                );
                
                break;
            }
        }
    }

    pub fn merge_buildings(
        &mut self,
        display: &glium::Display,
        positions: &mut Positions,
        indices: &mut FigureIndices<u16>,
        is_non_convex_hull: bool,
    ) {
        let choosed_vec = if is_non_convex_hull {
            &mut self.non_choosed_buildings
        } else {
            &mut self.choosed_buildings
        };

        let mut buildings = Vec::with_capacity(choosed_vec.len());  
        for (_, building_index) in choosed_vec.iter() {
            buildings.push(self.buildings[*building_index].clone());
        }

        
        let building = unsafe { ffi::BuildingsVec::new(buildings) };
        let layout = Layout::new::<ffi::BuildingsVec>();
        let building_vec = unsafe { alloc(layout).cast::<ffi::BuildingsVec>() };
        
        if building_vec.is_null() {
            panic!("Произошло переполнение памяти!");
        }

        unsafe { building_vec.write(building); };

        let building = if is_non_convex_hull {
            unsafe { *ffi::nc_hull_maker(building_vec, W).offset(0) }
        } else {
            unsafe { *ffi::merge_buildings(building_vec).offset(0) }
        };

        let building = building.repr_rust();
        
        choosed_vec.sort_by(
            |f, s| s.1.cmp(&f.1)
        );

        for (_, building_index) in choosed_vec.iter() {
            self.buildings.remove(*building_index);
        }

        self.buildings.push(building);

        *choosed_vec = Vec::new();

        self.set_positions(display, positions)
            .expect("Ошибка! Не удалось задать позици для зданий!");
        self.set_indices(display, indices)
            .expect("Ошибка! Не удалось задать индексы для позиций зданий!");
    }
}
