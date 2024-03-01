pub mod rotate_matrix {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for col in matrix {
            let rowrev = vec![col[0].clone()];
                // print!("{:?} ", rowrev);
            for row in rowrev.clone() {
                print!("{} ",row)
            };
            println!();
        };
    }
}
