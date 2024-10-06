use crate::objects::{Object, SquarePlane};

pub fn get_scene() -> Vec<Object> {
    let objects = vec![
        Object::SquarePlane(SquarePlane::new(
            (0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            2.5,
            "grass_top",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            2.5,
            "grass_top",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            2.5,
            "grass_top",
        )),
        Object::SquarePlane(SquarePlane::new(
            (0.0, 0.0, 2.5),
            (0.0, 1.0, 0.0),
            2.5,
            "grass_top",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, 0.0, 2.5),
            (0.0, 1.0, 0.0),
            2.5,
            "grass_top",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, 0.0, 2.5),
            (0.0, 1.0, 0.0),
            2.5,
            "grass_top",
        )),
        // Sides
        // FRONT THREE SIDES
        Object::SquarePlane(SquarePlane::new(
            (0.0, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "grass_side",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "grass_side",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, -1.25, -1.25),
            (0.0, 0.0, -1.0),
            2.5,
            "grass_side",
        )),

        // BACK THREE SIDES
        Object::SquarePlane(SquarePlane::new(
            (0.0, -1.25, 3.75),
            (0.0, 0.0, 1.0),
            2.5,
            "grass_side",
        )),
        Object::SquarePlane(SquarePlane::new(
            (2.5, -1.25, 3.75),
            (0.0, 0.0, 1.0),
            2.5,
            "grass_side",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-2.5, -1.25,3.75),
            (0.0, 0.0, 1.0),
            2.5,
            "grass_side",
        )),
        // RIGHT SIDES
        Object::SquarePlane(SquarePlane::new(
            (3.75, -1.25, 0.0),
            (1.0, 0.0, 0.0),
            2.5,
            "grass_side",
        )),
        Object::SquarePlane(SquarePlane::new(
            (3.75, -1.25, 2.5),
            (1.0, 0.0, 0.0),
            2.5,
            "grass_side",
        )),
        //LEFT SIDES
        Object::SquarePlane(SquarePlane::new(
            (-3.75, -1.25, 0.0),
            (-1.0, 0.0, 0.0),
            2.5,
            "grass_side",
        )),
        Object::SquarePlane(SquarePlane::new(
            (-3.75, -1.25, 2.5),
            (-1.0, 0.0, 0.0),
            2.5,
            "grass_side",
        )),

    ];
    objects
}
