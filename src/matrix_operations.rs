pub fn matrix_calculation(matrix: &[[f64; 3]; 3], vec: [f64; 3]) -> [f64; 3] {
    let mut result_vec = [0.; 3];
    let mut third_arr = [0.; 3];
    let mut second_arr = [0.; 3];
    let mut first_arr = [0.; 3];

    let first_second = matrix.split_last().unwrap();
    let third = first_second.1.split_last().unwrap();

    for (index, element) in first_second.0.iter().enumerate() {
        let result = element * vec[2];
        third_arr[index] = result;
    }

    for (index, element) in third.0.iter().enumerate() {
        let result = element * vec[1];
        second_arr[index] = result;
    }

    for (_, element) in third.1.iter().enumerate() {
        for (index, element) in element.iter().enumerate() {
            let result = element * vec[0];
            first_arr[index] = result;
        }
    }

    result_vec[0] = first_arr[0] + second_arr[0] + third_arr [0];
    result_vec[1] = first_arr[1] + second_arr[1] + third_arr [1];
    result_vec[2] = first_arr[2] + second_arr[2] + third_arr [2];

    result_vec
} 
