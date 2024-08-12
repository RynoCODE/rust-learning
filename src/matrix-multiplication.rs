use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the number of rows and columns for the first matrix (rows cols):");
    io::stdin().read_line(&mut input).unwrap();
    let dims: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (rows_a, cols_a) = (dims[0], dims[1]);
    
    let mut matrix_a = Vec::with_capacity(rows_a);
    for i in 0..rows_a {
        input.clear();
        println!("Enter row {} of the first matrix:", i + 1);
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix_a.push(row);
    }
    
    input.clear();
    println!("Enter the number of rows and columns for the second matrix (rows cols):");
    io::stdin().read_line(&mut input).unwrap();
    let dims: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (rows_b, cols_b) = (dims[0], dims[1]);
    
    if cols_a != rows_b {
        println!("Matrix multiplication is not possible with these dimensions.");
        return;
    }

    let mut matrix_b = Vec::with_capacity(rows_b);
    for i in 0..rows_b {
        input.clear();
        println!("Enter row {} of the second matrix:", i + 1);
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix_b.push(row);
    }

    let mut result = vec![vec![0; cols_b]; rows_a];
    
    for i in 0..rows_a {
        for j in 0..cols_b {
            result[i][j] = (0..cols_a)
                .map(|k| matrix_a[i][k] * matrix_b[k][j])
                .sum();
        }
    }

    println!("Result of matrix multiplication:");
    for row in result {
        println!("{:?}", row);
    }
}
