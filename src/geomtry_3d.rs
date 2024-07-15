
struct Cube {
    vertex_list: Vec<[f64; 3]>,
    edge_list: Vec<[[f64; 3]; 2]>,
}

impl Cube {
    fn new(ms: f64) -> Self {
        let mut vertex_list = Vec::new();
        let mut edge_list = Vec::new();

        vertex_list.push([0., 0., 0.]);
        vertex_list.push([ms, 0., 0.]);
        vertex_list.push([ms, ms, 0.]);
        vertex_list.push([0., ms, 0.]);
        vertex_list.push([0., 0., ms]);
        vertex_list.push([ms, 0., ms]);
        vertex_list.push([ms, ms, ms]);
        vertex_list.push([0., ms, ms]);

        edge_list.push([vertex_list[0], vertex_list[1]]);
        edge_list.push([vertex_list[1], vertex_list[2]]);
        edge_list.push([vertex_list[2], vertex_list[3]]);
        edge_list.push([vertex_list[3], vertex_list[0]]);
        
        edge_list.push([vertex_list[4], vertex_list[5]]);
        edge_list.push([vertex_list[5], vertex_list[6]]);
        edge_list.push([vertex_list[6], vertex_list[7]]);
        edge_list.push([vertex_list[7], vertex_list[4]]);
        
        edge_list.push([vertex_list[0], vertex_list[4]]);
        edge_list.push([vertex_list[1], vertex_list[5]]);
        edge_list.push([vertex_list[2], vertex_list[6]]);
        edge_list.push([vertex_list[3], vertex_list[7]]);

        Self { vertex_list, edge_list }
    }
}
