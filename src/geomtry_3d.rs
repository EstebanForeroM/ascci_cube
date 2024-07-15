use crate::{matrix_generation, matrix_operations};


pub struct Cube {
    pub vertex_list: Vec<[f64; 3]>,
}

impl Cube {
    pub fn new(ms: f64) -> Self {
        let mut vertex_list = Vec::new();

        vertex_list.push([0., 0., 0.]);
        vertex_list.push([ms, 0., 0.]);
        vertex_list.push([ms, ms, 0.]);
        vertex_list.push([0., ms, 0.]);
        vertex_list.push([0., 0., ms]);
        vertex_list.push([ms, 0., ms]);
        vertex_list.push([ms, ms, ms]);
        vertex_list.push([0., ms, ms]);

        Self { vertex_list }
    }

    pub fn get_edge_list(&self, translation_vector: [f64; 3]) -> Vec<[[f64; 3]; 2]> {
        let mut edge_list = Vec::new();

        edge_list.push([self.vertex_list[0], self.vertex_list[1]]);
        edge_list.push([self.vertex_list[1], self.vertex_list[2]]);
        edge_list.push([self.vertex_list[2], self.vertex_list[3]]);
        edge_list.push([self.vertex_list[3], self.vertex_list[0]]);
        
        edge_list.push([self.vertex_list[4], self.vertex_list[5]]);
        edge_list.push([self.vertex_list[5], self.vertex_list[6]]);
        edge_list.push([self.vertex_list[6], self.vertex_list[7]]);
        edge_list.push([self.vertex_list[7], self.vertex_list[4]]);
        
        edge_list.push([self.vertex_list[0], self.vertex_list[4]]);
        edge_list.push([self.vertex_list[1], self.vertex_list[5]]);
        edge_list.push([self.vertex_list[2], self.vertex_list[6]]);
        edge_list.push([self.vertex_list[3], self.vertex_list[7]]);

        for edge in edge_list.iter_mut() {
            for point in edge.iter_mut() {
                (*point)[0] = (*point)[0] + translation_vector[0];
                (*point)[1] = (*point)[1] + translation_vector[1];
                (*point)[2] = (*point)[2] + translation_vector[2];
            }
        }

        edge_list
    }

    pub fn rotate_cube(&mut self, rot_vec: [f64; 3]) {
        let rotation_x_matrix = matrix_generation::generate_rotationx_matrix(rot_vec[0]);
        let rotation_y_matrix = matrix_generation::generate_rotationy_matrix(rot_vec[1]);
        let rotation_z_matrix = matrix_generation::generate_rotationz_matrix(rot_vec[2]);

        for vertex in self.vertex_list.iter_mut() {
            *vertex = matrix_operations::matrix_calculation(&rotation_x_matrix, *vertex);
            *vertex = matrix_operations::matrix_calculation(&rotation_y_matrix, *vertex);
            *vertex = matrix_operations::matrix_calculation(&rotation_z_matrix, *vertex);
        }
    }
}
