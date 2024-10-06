use std::char::MAX;

use nalgebra_glm::normalize;
use nalgebra_glm::Vec3;
use crate::light;
use crate::objects::Sphere;
use crate::objects::RayIntersect;
use crate::objects::Object;
use crate::color::Color;
use crate::materials::Intersect;
use crate::Light;

const MAX_DEPTH: u32 = 3;
const SKYBOX_COLOR: u32 = 0x7ed0d9;
const ORIGIN_BIAS: f32 = 1e-4;

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Object],
    lights: &[Light],
    depth: u32,
) -> u32{
    if depth>MAX_DEPTH{
        return SKYBOX_COLOR
    }
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
        return SKYBOX_COLOR
    }

    let view_direction = (ray_origin - intersect.point).normalize();
    let mut total_diffuse = Color::from_hex(0x000000);
    let mut total_specular = Color::from_hex(0x000000);
    
    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let reflect_dir = reflect(&light_dir, &intersect.normal).normalize();
        
        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Diffuse
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse_color = intersect.material.get_diffuse(intersect.u, intersect.v);
        total_diffuse = total_diffuse + diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Specular
        let specular_intensity = view_direction.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        total_specular = total_specular+ light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;
    }

    let mut reflect_color = 0x000000;
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0{
        let reflect_dir= reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth+1);
    }

    let mut refract_color = 0x000000;
    
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refractive_index).normalize();
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, depth+1)
    }


    let total_color = Color::to_hex(&(( total_diffuse* (1.0 - reflectivity - transparency)) 
                 + (Color::from_hex(reflect_color) * reflectivity)
                 + (Color::from_hex(refract_color) * transparency)));
    total_color
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3{
    incident -2.0*incident.dot(normal)*normal
}

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // Ray is entering the object
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // Ray is leaving the object
        n_cosi = cosi;
        eta = eta_t;  // Assuming it's going back into air with index 1.0
        n_normal = *normal;
    }
    
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    
    if k < 0.0 {
        // Total internal reflection
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Object],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point;
    let mut shadow_intensity = 0.0;
    let light_distance = (light.position - intersect.point).magnitude()*500.0;
    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            shadow_intensity = (1.0/light_distance).max(1.0);
            break;
        }
    }
    shadow_intensity
}