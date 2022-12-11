use std::borrow::Borrow;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

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

    let counter = get_visible_trees(&matrix);
    println!("Counter: {counter}");

    let viewing_distance = get_max_viewing_distance(&matrix);
    println!("Viewing distance: {viewing_distance}");
}

fn get_visible_trees(matrix: &Vec<Vec<u32>>) -> i32 {

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

    return counter;
}

fn get_max_viewing_distance(matrix: &Vec<Vec<u32>>) -> i32 {

    let mut max_viewing_distance = 0;
    let mut max_element: String = String::from("");

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            let elem = matrix[i][j];
            //println!("Elem: {elem}");
            let mut viewing_distance = 0;

            // 1st check
            let mut vd_top = 0;
            if i > 0 {
                for ii in (0..i).rev() {
                    vd_top += 1;
                    if matrix[ii][j] >= matrix[i][j] {
                        break;
                    }
                }
            }

            // 2nd check
            let mut vd_bottom = 0;
            if i<matrix.len()-1 {
                for ii in i+1..matrix.len() {
                    vd_bottom += 1;
                    if matrix[ii][j] >= matrix[i][j] {
                        break;
                    }
                }
            }

            // 3rd check
            let mut vd_left = 0;
            if j > 0 {
                for jj in (0..j).rev() {
                    vd_left += 1;
                    if matrix[i][jj] >= matrix[i][j] {
                        break;
                    }
                }
            }

            // 4th check
            let mut vd_right = 0;
            if j<matrix.len()-1 {
                for jj in j+1..matrix.len() {
                    vd_right += 1;
                    if matrix[i][jj] >= matrix[i][j] {
                        break;
                    }
                }
            }

            viewing_distance = vd_top * vd_bottom * vd_left * vd_right;

            if viewing_distance > max_viewing_distance {
                max_viewing_distance = viewing_distance;
                max_element = format!("elem: {0} {1}", i.to_string(), j.to_string());
            }
        }
    }
    println!("{}", max_element);
    return max_viewing_distance;
}
