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

    let mut h_position: (i32, i32) = (0, 0);
    let mut t_position: (i32, i32) = (0, 0);

    let mut different_t_positions: HashSet<String> = HashSet::new();

    for l in buffered.lines() { // re-assign in a simple vector so that i can use peekable correctly after
        let line = l.unwrap_or_default();
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
