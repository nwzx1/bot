pub mod rotate_matrix_image;

use rotate_matrix_image::rotate_matrix as rmi;

fn main() {
    rmi::rotate(&mut vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ])
}
