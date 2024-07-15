use std::usize;

use super::Pixel;

pub fn line_renderer(matrix: &mut Vec<Vec<Pixel>>, points: [[f64; 3]; 2]) {
    let point1 = points[0];
    let point2 = points[1];
    
    let y0 = point2[1];
    let x0 = point2[0];

    let m = (point1[1] - point2[1]) / (point1[0] - point2[0]);
    let b = y0 - (m * x0);

    let y = |x: f64| (m * x) + b;

    let mut total = point1[0];
    let mut other = point2[0];

    if x0 == total {
        let (lesser_y, greater_y) = if y0 > point1[1] {
            (point1[1], y0) 
        } else {
            (y0, point1[1])
        };

        let mut y_actual = lesser_y;
        while y_actual < matrix.len() as f64 && y_actual < greater_y {
            matrix[y_actual as usize][x0 as usize] = Pixel::Full;
            y_actual += 1.;
        }
        return
    }

    if total < other {
    } else {
        total = point2[0];
        other = point1[0];
    }

    let dx = 0.001;

    while total<= other {
        total += dx;
        let y = y(total) as usize;

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
