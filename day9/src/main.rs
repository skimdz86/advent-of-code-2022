use std::collections::HashSet;
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

    let mut line_vec: Vec<String> = Vec::new();
    for l in buffered.lines() {
        let line = l.unwrap_or_default();
        line_vec.push(line);
    }

    short_rope(&line_vec);

    multi_knot_rope(&line_vec);

}

fn short_rope (line_vec: &Vec<String>) {
    let mut h_position: (i32, i32) = (0, 0);
    let mut t_position: (i32, i32) = (0, 0);

    let mut different_t_positions: HashSet<String> = HashSet::new();

    for line in line_vec { // re-assign in a simple vector so that i can use peekable correctly after
        let line_pair: Vec<&str> = line.split(" ").collect();
        let direction = line_pair[0];
        let steps: i32 = line_pair[1].parse().unwrap();

        for i in 0..steps {// the tail checks the distance for each move, step by step. Can not move head for more than 1 step at a time
            match direction {
                "L" => {
                    h_position.1 = h_position.1 - 1;
                },
                "R" => {
                    h_position.1 = h_position.1 + 1;
                },
                "U" => {
                    h_position.0 = h_position.0 - 1;
                },
                "D" => {
                    h_position.0 = h_position.0 + 1;
                },
                _ => println!("Something's wrong: direction={direction}")
            }

            // to understand when the tail moves, it's enough that the H indexes be >=T+2 or <=T-2 in every direction (it's enough 1 of the 2 direction >=T+/-2)
            if h_position.0 >= t_position.0 + 2 {
                // H is too down
                t_position.0 += 1;
                move_second_direction_horizontal(h_position, &mut t_position);
            } else if h_position.0 <= t_position.0 - 2 {
                // H is too up
                t_position.0 -= 1;
                move_second_direction_horizontal(h_position, &mut t_position);
            } else if h_position.1 >= t_position.1 + 2 {
                // H is too right
                t_position.1 += 1;
                move_second_direction_vertical(h_position, &mut t_position);
            } else if h_position.1 <= t_position.1 - 2 {
                // H is too left
                t_position.1 -= 1;
                move_second_direction_vertical(h_position, &mut t_position);
            }

            println!("New H position: ({}, {})", h_position.0, h_position.1);
            println!("New T position: ({}, {})", t_position.0, t_position.1);

            let mut hash_key: String = String::from("");
            hash_key.push_str(&t_position.0.to_string());
            hash_key.push_str("-");
            hash_key.push_str(&t_position.1.to_string());
            let result = different_t_positions.insert(hash_key);
            println!("Inserted: {result}");
        }

    }

    println!("Different tail positions: {}", different_t_positions.len());
}

fn multi_knot_rope(line_vec: &Vec<String>) {
    let mut h_position: (i32, i32) = (0, 0);
    let mut t_positions: Vec<(i32, i32)> = vec![(0,0); 9];

    let mut different_t_positions: HashSet<String> = HashSet::new();

    for line in line_vec { // re-assign in a simple vector so that i can use peekable correctly after

        //println!("Line {line}");
        let line_pair: Vec<&str> = line.split(" ").collect();
        let direction = line_pair[0];
        let steps: i32 = line_pair[1].parse().unwrap();

        for i in 0..steps {// the tail checks the distance for each move, step by step. Can not move head for more than 1 step at a time
            //println!("Step {i}");
            match direction {
                "L" => {
                    h_position.1 = h_position.1 - 1;
                },
                "R" => {
                    h_position.1 = h_position.1 + 1;
                },
                "U" => {
                    h_position.0 = h_position.0 - 1;
                },
                "D" => {
                    h_position.0 = h_position.0 + 1;
                },
                _ => println!("Something's wrong: direction={direction}")
            }

            // loop over all the knots
            let mut previous_k_position: (i32, i32) = (h_position.0, h_position.1);
            //println!("New HEAD position {} {}", previous_k_position.0, previous_k_position.1);
            let mut index = 1;
            for tuple in t_positions.clone() {
                let mut k_position = tuple;
                //println!("Tuple {} {}", tuple.0, tuple.1);
                // to understand when the tail moves, it's enough that the H indexes be >=T+2 or <=T-2 in every direction (it's enough 1 of the 2 direction >=T+/-2)
                if previous_k_position.0 >= k_position.0 + 2 {
                    // H is too down
                    k_position.0 += 1;
                    move_second_direction_horizontal(previous_k_position, &mut k_position);
                } else if previous_k_position.0 <= k_position.0 - 2 {
                    // H is too up
                    k_position.0 -= 1;
                    move_second_direction_horizontal(previous_k_position, &mut k_position);
                } else if previous_k_position.1 >= k_position.1 + 2 {
                    // H is too right
                    k_position.1 += 1;
                    move_second_direction_vertical(previous_k_position, &mut k_position);
                } else if previous_k_position.1 <= k_position.1 - 2 {
                    // H is too left
                    k_position.1 -= 1;
                    move_second_direction_vertical(previous_k_position, &mut k_position);
                }

                //println!("New {} position: ({}, {})", index-1, previous_k_position.0, previous_k_position.1);
                //println!("New {index} position: ({}, {})", k_position.0, k_position.1);

                previous_k_position = (k_position.0, k_position.1);
                t_positions[index-1] = (k_position.0, k_position.1); // update vec of knot positions

                if index == 9 {
                    let mut hash_key: String = String::from("");
                    hash_key.push_str(&k_position.0.to_string());
                    hash_key.push_str("-");
                    hash_key.push_str(&k_position.1.to_string());
                    let result = different_t_positions.insert(hash_key);
                    //println!("Inserted: {result}");
                }
                index += 1;
            }
        }

    }

    println!("Different 9-tails positions: {}", different_t_positions.len());
}

fn move_second_direction_horizontal(h_position: (i32, i32), t_position: &mut (i32, i32)){
    if h_position.1 > t_position.1 {
        t_position.1 += 1;
    } else if h_position.1 < t_position.1 {
        t_position.1 -= 1;
    } else {
        // same line, do nothing
    }
}

fn move_second_direction_vertical(h_position: (i32, i32), t_position: &mut (i32, i32)){
    if h_position.0 > t_position.0 {
        t_position.0 += 1;
    } else if h_position.0 < t_position.0 {
        t_position.0 -= 1;
    } else {
        // same line, do nothing
    }
}
