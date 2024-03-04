pub mod rotate_matrix_image;

use rotate_matrix_image::rotate_matrix as rmi;

fn main() {
    let matrx3d = &mut vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    let matrx2d = &mut vec![
        vec![1, 2, 3], 
        vec![4, 5, 6], 
        vec![7, 8, 9]
    ];

    let ans = rmi::rotate(matrx2d);

    println!("{:?}", ans)
}
