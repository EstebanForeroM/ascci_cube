use std::f64::consts::PI;

pub fn generate_rotationx_matrix(degrees: f64) -> [[f64; 3]; 3] {
    let radians = degrees * PI / 180.0;
    let cos_theta = radians.cos();
    let sin_theta = radians.sin();

    [
        [1.0, 0.0, 0.0],
        [0.0, cos_theta, -sin_theta],
        [0.0, sin_theta, cos_theta],
    ]
}

pub fn generate_rotationy_matrix(degrees: f64) -> [[f64; 3]; 3] {
    let radians = degrees * PI / 180.0;
    let cos_theta = radians.cos();
    let sin_theta = radians.sin();

    [
        [cos_theta, 0.0, sin_theta],
        [0.0, 1.0, 0.0],
        [-sin_theta, 0.0, cos_theta],
    ]
}

pub fn generate_rotationz_matrix(degrees: f64) -> [[f64; 3]; 3] {
    let radians = degrees * PI / 180.0;
    let cos_theta = radians.cos();
    let sin_theta = radians.sin();

    [
        [cos_theta, -sin_theta, 0.0],
        [sin_theta, cos_theta, 0.0],
        [0.0, 0.0, 1.0],
    ]
}

