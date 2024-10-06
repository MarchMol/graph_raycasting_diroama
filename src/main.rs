mod framebuffer;
mod color;
use camera::Camera;
use color::Color;
use light::Light;
use nalgebra_glm::{normalize, Vec3};
use objects::{Sphere, SquarePlane};
mod camera;
mod materials;
mod objects;
mod ray_caster;
mod light;
use std::time::{Duration};
use minifb::{Key, Window, WindowOptions};
mod scene;
mod texture;
fn main() {
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let frame_delay = Duration::from_millis(0);
    let objects = scene::get_scene();
    
    let mut window = Window::new(
        "Raytracing - Test",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();
    let mut camera = Camera{
        eye: Vec3::new(0.0,0.0,-10.0), 
        center:  Vec3::new(0.0,0.0,0.0), 
        up: Vec3::new(0.0,-1.0,0.0),
        has_changed: true};

    let mut light = Light::new(
        Vec3::new(0.0, 5.0, 10.0),
        Color::new(255, 255, 255),
        1.0
    );
    let mut light_change = true;
    while window.is_open(){
        // closing game
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::W) {
           camera.orbit(0.0, -0.2)
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, 0.2)
         }
         if window.is_key_down(Key::A) {
            camera.orbit(-0.2, 0.0)
         }
         if window.is_key_down(Key::D) {
             camera.orbit(0.2, 0.0)
          }
          if window.is_key_down(Key::Q) {
            camera.zoom(0.2);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(-0.2);
        }

          
          
        if window.is_key_down(Key::Up) {
            light.position.y = light.position.y +0.2;
            light_change = true;
         }
         if window.is_key_down(Key::Down) {
            light.position.y = light.position.y -0.2;
            light_change = true;
          }
          if window.is_key_down(Key::Left) {
            light.position.x = light.position.x -0.2;
            light_change = true;
          }
          if window.is_key_down(Key::Right) {
            light.position.x = light.position.x +0.2;
            light_change = true;
           }
           if window.is_key_down(Key::O) {
            light.position.z = light.position.z +0.2;
            light_change = true;
          }
          if window.is_key_down(Key::L) {
            light.position.z = light.position.z -0.2;
            light_change = true;
           }
        if camera.check_if_change() || light_change{
            render(&mut framebuffer, &objects, &camera, &light);
            light_change = false;
        }
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


pub fn render(framebuffer: &mut framebuffer::Framebuffer, objects: &[objects::Object], camera: &Camera, light: &Light) {
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
            let ray_direction = normalize(
                &camera.basis_change(&Vec3::new(screen_x, screen_y, -1.0))
            );


            // Cast the ray and get the pixel color
            let pixel_color:u32 = ray_caster::cast_ray(
                
                &camera.eye, &ray_direction,
                 objects,&light,
                0);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}