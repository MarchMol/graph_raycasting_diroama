use nalgebra_glm::Vec3;
use crate::objects::Sphere;
use crate::objects::RayIntersect;
use crate::color::Color;
use crate::ray_intersect::Intersect;
pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Sphere]
) -> u32{
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects{
        let tem = object.ray_intersect(ray_origin, ray_direction);
        if tem.is_intersecting &&
        tem.distance<zbuffer{
            zbuffer = tem.distance;
            intersect = tem;
        }
    }
    if !intersect.is_intersecting{
        return 0x000000;
    }
    Color::to_hex(&intersect.material.diffuse)
}