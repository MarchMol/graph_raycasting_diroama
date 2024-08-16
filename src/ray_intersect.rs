use nalgebra_glm::Vec3;
use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}
impl Material{
    pub fn new( material: &str)-> Self{
        
        let mut hex:u32 = 0x000000;
        
        if material=="fur"{
            hex = 0xa6a6a6;
        } else if material=="white"{
            hex= 0xffffff;
        }else if material=="nose"{
            hex = 0xba598b;
        }
        let mut diffuse = Color::from_hex(hex);
        Material{
            diffuse
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    // pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material) -> Self {
    //     Intersect {
    //         distance,
    //         is_intersecting: true,
    //         material,
    //     }
    // }

        pub fn new(distance: f32, is_intersecting: bool,material: Material) -> Self {
        Intersect {
            distance,
            is_intersecting,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material{
              diffuse: Color::new(0, 0, 0),
            },
        }
    }
}

pub trait RayIntersect {
  fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}