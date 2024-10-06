use nalgebra_glm::Vec3;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::color::Color;
use crate::texture::Texture;
use crate::light::Light;

static GRASS_SIDE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/grass_side.bmp")));
static GRASS_TOP: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/grass_top.bmp")));
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32;4],
    pub refractive_index: f32,
    pub is_emissive: bool,
    pub emissive_light: Option<Light>
}
impl Material{
    pub fn new( material: &str)-> Self{
        let mut albedo = [0.9, 0.1, 0.0, 0.0];
        let mut hex_color:u32 = 0x000000;
        let mut specular:f32 = 10.0;
        let mut is_emissive = false;
        let mut emissive_light = None;
        let mut refractive_index = 0.1;

        if material=="normal"{
            hex_color = 0xa6a6a6;
            specular=5.0;
            albedo = [0.9, 0.1, 0.4, 0.5]
        } else if material=="emissive"{
            hex_color = 0xa6a6a6;
            specular=5.0;
            albedo = [0.9, 0.1, 0.0, 0.5];
            is_emissive= true;
            emissive_light = Some(Light::new(
                Vec3::new(0.0, 5.0, 10.0),
                Color::from_hex(0xff9100),
                1.0
            ));
        }
        let diffuse = Color::from_hex(hex_color);
        Material{
            diffuse,
            specular,
            albedo,
            refractive_index,
            is_emissive,
            emissive_light
        }
    }

    pub fn get_diffuse (&self, u: f32, v: f32) -> Color{
        let x =(u  * (GRASS_TOP.width as f32-1.0)) as usize;
        let y =(v  * (GRASS_TOP.height as f32-1.0)) as usize;
        GRASS_TOP.get_color(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32
}

impl Intersect {
        pub fn new(point: Vec3, normal: Vec3, distance: f32, is_intersecting: bool,material: Material, u: f32, v: f32) -> Self {
        Intersect {
            point, 
            normal,
            distance,
            is_intersecting,
            material,
            u, v
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material{
              diffuse: Color::new(0, 0, 0),
              specular: 0.0,
              albedo: [0.0, 0.0, 0.0, 0.0],
              refractive_index: 0.0,
              is_emissive: false,
              emissive_light: None
            },
            u: 0.0,
            v: 0.0
        }
    }
}

pub trait RayIntersect {
  fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}