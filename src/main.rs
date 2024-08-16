mod framebuffer;
mod color;
use color::Color;
use nalgebra_glm::{normalize, Vec3};

mod ray_intersect;
mod objects;
mod ray_caster;
use std::time::{Duration};
use minifb::{Key, Window, WindowOptions};
fn main() {
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let frame_delay = Duration::from_millis(16);
    let ear_r = objects::Sphere::new(
        Vec3::new(-5.0,4.0,15.0),
        3.0,
        ray_intersect::Material::new("fur")
    );
    let in_ear_r = objects::Sphere::new(
        Vec3::new(-5.0,4.0,14.0),
        2.0,
        ray_intersect::Material::new("nose")
    );
    let ear_l = objects::Sphere::new(
        Vec3::new(5.0,4.0,15.0),
        3.0,
        ray_intersect::Material::new("fur")
    );
    let in_ear_l = objects::Sphere::new(
        Vec3::new(5.0,4.0,14.0),
        2.0,
        ray_intersect::Material::new("nose")
    );
    let head = objects::Sphere::new(
        Vec3::new(0.0,0.0,15.0),
        5.0,
        ray_intersect::Material::new("fur")
    );
    let eye_r = objects::Sphere::new(
        Vec3::new(2.4,0.8,13.0),
        1.0,
        ray_intersect::Material::new("")
    );
    let eye_l = objects::Sphere::new(
        Vec3::new(-2.4,0.8,13.0),
        1.0,
        ray_intersect::Material::new("")
    );
    let eye_shine_l = objects::Sphere::new(
        Vec3::new(-2.4,1.2,12.0),
        0.2,
        ray_intersect::Material::new("white")
    );
    let eye_shine_r = objects::Sphere::new(
        Vec3::new(2.0,1.2,12.0),
        0.2,
        ray_intersect::Material::new("white")
    );

    let nose = objects::Sphere::new(
        Vec3::new(0.0,-1.5,11.0),
        0.5,
        ray_intersect::Material::new("nose")
    );

    let objects = [
        ear_l, ear_r, in_ear_r,in_ear_l,
        head, 
        eye_l, eye_r, eye_shine_l, eye_shine_r, 
        nose,
        ];
    let mut window = Window::new(
        "Raytracing - Test",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open(){
        // closing game
        if window.is_key_down(Key::Escape) {
            break;
        }

        render(&mut framebuffer, &objects);
        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}


pub fn render(framebuffer: &mut framebuffer::Framebuffer, objects: &[objects::Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Cast the ray and get the pixel color
            let pixel_color:u32 = ray_caster::cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}