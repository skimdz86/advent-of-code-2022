use std::borrow::Borrow;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

pub struct Matrix {
    rows: [[usize; 3]; 3],
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", file_path, Error::to_string(&why)),
        Ok(file) => file,
    };

    let buffered = BufReader::new(file);

    let mut matrix_size = 0;
    let lines = buffered.lines();
    let mut line_vector: Vec<String>  = Vec::new();
    for line in lines {
        let l = line.unwrap();
        matrix_size = l.len();
        line_vector.push(l);
    }

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; matrix_size]; matrix_size];

    let mut i: usize = 0;
    for line in line_vector {
        let unwrapped_line = line;
        let mut chars = unwrapped_line.chars();

        let mut j: usize = 0;
        for c in chars {
            matrix[i][j] = c.to_digit(10).unwrap();
            j += 1;
        }
        i += 1;
    }

    let mut counter = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            let elem = matrix[i][j];
            //println!("Elem: {elem}");

            let mut visible_trees = 0;

            if i == 0 || j == 0 || i == matrix.len()-1 || j == matrix.len()-1 {
                counter += 1;
            } else {
                // 1st check
                let mut visible = true;
                for ii in 0..i {
                    if matrix[ii][j] >= matrix[i][j] {
                        visible = false;
                    }
                }
                if visible == true {
                    println!("Matrix[{}][{}] visible from the top", i, j);
                    counter += 1;
                    continue
                }

                // 2nd check
                let mut visible = true;
                for ii in i+1..matrix.len() {
                    if matrix[ii][j] >= matrix[i][j] {
                        visible = false;
                    }
                }
                if visible == true {
                    println!("Matrix[{}][{}] visible from the bottom", i, j);
                    counter += 1;
                    continue
                }

                // 3rd check
                let mut visible = true;
                for jj in 0..j {
                    if matrix[i][jj] >= matrix[i][j] {
                        visible = false;
                    }
                }
                if visible == true {
                    println!("Matrix[{}][{}] visible from the left", i, j);
                    counter += 1;
                    continue
                }

                // 4th check
                let mut visible = true;
                for jj in j+1..matrix.len() {
                    if matrix[i][jj] >= matrix[i][j] {
                        visible = false;
                    }
                }
                if visible == true {
                    println!("Matrix[{}][{}] visible from the right", i, j);
                    counter += 1;
                    continue
                }
            }

        }
    }

    println!("Counter: {counter}");
}
