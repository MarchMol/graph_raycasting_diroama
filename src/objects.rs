use std::f32::consts::E;

use nalgebra_glm::{dot, Vec3};
use crate::ray_intersect;
use crate::ray_intersect::Material;
use crate::ray_intersect::Intersect;
pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}
impl Sphere{
    pub fn new(
         center: Vec3,
         radius: f32,
         material: Material
    ) -> Self{
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl RayIntersect for Sphere{
    fn ray_intersect(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Intersect {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0* dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius*self.radius;

        let discriminant = b*b -4.0*a*c;
        if discriminant>0.0{
            return Intersect::new(oc.magnitude(), true,self.material)
        } else{
            return Intersect::new(oc.magnitude(),false, self.material)
        }
    }
}
// pub struct Object {
//     // fields
// }

// impl Object{
//     pub fn ray_intersect(
//         &self,
//         ray_origin: &Vec3,
//         ray_direction: &Vec3,
//     ) -> Intersect{
//         true
//     }
// }

pub trait RayIntersect {
    fn ray_intersect(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Intersect;
}