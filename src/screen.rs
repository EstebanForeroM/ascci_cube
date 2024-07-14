
pub enum Pixel {
    Full,
    Empty,
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self {
            Pixel::Full => "\x1b[47m \x1b[0m",
            Pixel::Empty => "\x1b[46m \x1b[0m",
        }.to_string()
    }
}

pub fn default_pixel_matrix(hight: usize, width: usize) -> Vec<Vec<Pixel>> {
    let mut blank_matrix = Vec::with_capacity(hight);

    for _ in 0..hight {
        let mut row_vector = Vec::with_capacity(width);
        for _ in 0..width {
            row_vector.push(Pixel::Empty);
        }
        blank_matrix.push(row_vector);
    }

    blank_matrix
}

pub fn show_pixel_matrix(matrix: &Vec<Vec<Pixel>>) {
    print!("\n");
    for row in matrix.iter().rev() {
        for pixel in row {
            print!("{}", pixel.to_string());
        }
        print!("\n");
    }   
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pixel_matrix_string() {
        todo!();
    }
}
