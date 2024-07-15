use std::usize;

use super::Pixel;

pub fn line_renderer(matrix: &mut Vec<Vec<Pixel>>, points: [[f64; 3]; 2]) {
    let point1 = points[0];
    let point2 = points[1];
    
    let y0 = point2[1];
    let x0 = point2[0];
    println!("y0 = {y0}");
    println!("x0 = {x0}");

    let m = (point1[1] - point2[1]) / (point1[0] - point2[0]);
    let b = y0 - (m * x0);
    println!("m = {m}");
    println!("b = {b}");

    let y = |x: f64| (m * x) + b;

    let mut total = point1[0];
    let mut other = point2[0];

    if total < other {
        println!("total is = {total} and other is = {other}");
    } else {
        total = point2[0];
        other = point1[0];
    }

    let dx = 0.001;

    while total<= other {
        total += dx;
        let y = y(total) as usize;

        println!("total = {total}, y = {y}");
        if y >= matrix.len() || total as usize > matrix[0].len() { break };
        matrix[y][total as usize] = Pixel::Full;
    }
}

#[cfg(test)]
mod test {
    use super::{*,super::*};

    #[test]
    fn line_render_test() {
        let mut matrix = screen::default_pixel_matrix(80, 100);
        let points = [[89., 70., 0.],[2., 8., 0.]];
        line_renderer(&mut matrix, points);
        screen::show_pixel_matrix(&matrix);
        assert!(false);
    }
}
