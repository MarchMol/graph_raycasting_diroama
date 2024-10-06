use crate::materials::Material;
use crate::objects::{Object, SquarePlane};
use nalgebra_glm::Vec3;

pub fn get_scene() -> Vec<Object> {
    let objects = vec![
        Object::SquarePlane(SquarePlane::new(
            (0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (0.0, 0.0, 2.5),
            (0.0, 1.0, 0.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, 0.0, 2.5),
            (0.0, 1.0, 0.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, 0.0, 2.5),
            (0.0, 1.0, 0.0),
            2.5,
            "normal",
        )),
        // Sides
        // FRONT THREE SIDES
        Object::SquarePlane(SquarePlane::new(
            (0.0, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "normal",
        )),
        
        // BACK THREE SIDES
        Object::SquarePlane(SquarePlane::new(
            (0.0, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "normal",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "normal",
        )),
    ];
    objects
}
