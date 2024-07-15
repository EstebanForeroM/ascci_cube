use std::{thread, time::Duration};

use line_renderer::line_renderer;
use rand::Rng;
use screen::Pixel;

pub mod screen;
mod line_renderer;
pub mod matrix_operations;
pub mod matrix_generation;
mod geomtry_3d;

fn main() {
    screen_example();
}

fn screen_example() {
    let mut cube = geomtry_3d::Cube::new(60.);
    cube.rotate_cube([9., 9., 9.]);

    let mut initial_x = 0.;
    let mut delta_x = 2.;

    loop {
        let mut pixel_matrix = screen::default_pixel_matrix(160, 600);

        initial_x += delta_x;
        if initial_x > 500. {
            initial_x -= delta_x;
            delta_x = -delta_x
        }

        for edge in cube.get_edge_list([initial_x, 50., 0.]) {
            line_renderer(&mut pixel_matrix, edge);
        }

        screen::clean_screen();
        screen::show_pixel_matrix(&pixel_matrix);

        const VELOCITY: f64 = 3.3;
        let rand_rot_x = rand::thread_rng().gen::<f64>() * VELOCITY;
        let rand_rot_y = rand::thread_rng().gen::<f64>() * VELOCITY;
        let rand_rot_z = rand::thread_rng().gen::<f64>() * VELOCITY;
        cube.rotate_cube([rand_rot_x, rand_rot_y, rand_rot_z]);
        thread::sleep(Duration::from_millis(13))
    }
}
