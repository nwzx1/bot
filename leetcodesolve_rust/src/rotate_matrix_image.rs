pub mod rotate_matrix {

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut arr = matrix.clone();

        let n = arr.len();
        for col in 0..n {
            for row in 0..col + 1 {
                let mutmex = arr[col][row].clone();
                arr[col][row] = arr[row][col].clone();
                arr[row][col] = mutmex.clone();
            }
        }

        let mut arr2 = arr.clone();
        let mut s = 0;
        let mut e = n - 1;
        while s < e {
            // Swap the elements of the start col with end col
            for i in 0..n {
                let temp = arr2[i][s];
                arr2[i][s] = arr2[i][e];
                arr2[i][e] = temp;
            }
            s += 1;
            e -= 1;
        }
        return arr2;
    }
}
