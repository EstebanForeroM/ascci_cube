use screen::Pixel;

mod screen;
mod line_renderer;
mod matrix_operations;

fn main() {

}

fn screen_example() {
    let mut pixel_matrix = screen::default_pixel_matrix(30, 30);
    pixel_matrix[0][0] = Pixel::Full;
    screen::show_pixel_matrix(&pixel_matrix);
}
