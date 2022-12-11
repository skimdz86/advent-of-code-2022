use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use regex::Regex;
use std::collections::LinkedList;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let MODE_DEFAULT: String = String::from("SINGLE");
    let move_mode = args.get(2).or(Some(&MODE_DEFAULT)).unwrap(); // SINGLE or MULTIPLE

    println!("In file {}, move_mode={}", file_path, move_mode);

    //let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", file_path, Error::to_string(&why)),
        Ok(file) => file,
    };

    let buffered = BufReader::new(file);

    let re = Regex::new("[ \\[\\]]*").unwrap();

    let mut all_stacks: Vec<Vec<char>> = Vec::new();
    let mut cargo_rows_stacked: Vec<String> = Vec::new();

    let mut index = 0;
    for row in buffered.lines() {
        let line = row.unwrap_or_default();
        //println!("{}", line);

        if index < 8 {
            cargo_rows_stacked.push(line);
        } else if index == 8 {
            println!("Cargo rows: {}", cargo_rows_stacked.len());

            // find string indexes to understand the position of the crates ( index for first column is 1, for the second is 5, etc)
            /*
                                    [R] [J] [W]
                        [R] [N]     [T] [T] [C]
            [R]         [P] [G]     [J] [P] [T]
            [Q]     [C] [M] [V]     [F] [F] [H]
            [G] [P] [M] [S] [Z]     [Z] [C] [Q]
            [P] [C] [P] [Q] [J] [J] [P] [H] [Z]
            [C] [T] [H] [T] [H] [P] [G] [L] [V]
            [F] [W] [B] [L] [P] [D] [L] [N] [G]
             1   2   3   4   5   6   7   8   9

            */
            let mut crate_stack_indexes: LinkedList<usize> = LinkedList::new();
            for (i, c) in line.chars().enumerate() {
                // do something with character `c` and index `i`
                if c != ' ' {
                    crate_stack_indexes.push_back(i);
                    all_stacks.push(Vec::new()); // create a new stack for each index found
                }
            }

            // populate all the vectors from the first 8 rows
            for index_crs in 0..cargo_rows_stacked.len() {
                // since it's a stack, the last inserted (the lowest) is the first retrieved (so i'm getting the low level first)
                // the first retrieved is for example: [F] [W] [B] [L] [P] [D] [L] [N] [G]
                let stacked_row = cargo_rows_stacked.pop();
                let crate_row = stacked_row.unwrap(); // e.g. [R]         [P] [G]     [J] [P] [T]
                for (j, &csi) in crate_stack_indexes.iter().enumerate() {
                    let chars: Vec<char> = crate_row.chars().collect();
                    let &current_value = chars.get(csi).unwrap();
                    if current_value != ' ' {
                        // push the value only if present
                        add_char_to_stack(&mut all_stacks, j, current_value);
                    }
                }
            }

            // print all the stacks re-composed
            for (i, s) in all_stacks.iter().enumerate() {
                println!("{i}th stack: {:?}", s.to_vec());
            }

        } else if index >= 10 {

            // line example: "move 2 from 2 to 8"

            let fields = line.split(" ");
            /*
            println!("LINE: {line}");
            for (_, s) in fields.clone().enumerate(){
                println!("FIELDS: {}", s);
            }
            */

            let block_num: usize = fields.clone().nth(1).unwrap().parse().unwrap();
            let stack_from: usize = fields.clone().nth(3).unwrap().parse().unwrap();
            let stack_to: usize = fields.clone().nth(5).unwrap().parse().unwrap();

            move_block(&mut all_stacks, stack_from - 1, stack_to - 1, block_num, move_mode); // -1, the array starts at 0 but the indicated position not

        } else {
            println!("Separator lines");
        }

        index+=1;
    }

    // print all the stacks at the final stage
    println!("============= Final result =============");
    for (i, s) in all_stacks.iter().enumerate() {
        println!("{i}th stack: {:?}", s.to_vec());
    }
}

fn move_block(matrix: &mut Vec<Vec<char>>, stack_from: usize, stack_to: usize, block_num: usize, move_mode: &String) {

    if move_mode == "MULTIPLE" {

        let mut intermediate_stack: Vec<char> = Vec::new();
        for i in 0..block_num {
            let block = matrix[stack_from].pop().unwrap();
            intermediate_stack.push(block);
        }

        for i in 0..block_num {
            let block = intermediate_stack.pop().unwrap();
            matrix[stack_to].push(block);
        }

    } else {
        for i in 0..block_num {
            let block = matrix[stack_from].pop().unwrap();
            matrix[stack_to].push(block);
        }
    }

    /*
    println!("####### NEW situation ##########");
    for (i, s) in matrix.iter().enumerate() {
        println!("{i}th stack: {:?}", s.to_vec());
    }
    println!("#####################################");
    */
}

fn add_char_to_stack(matrix: &mut Vec<Vec<char>>, j: usize, new_value: char){

    matrix[j].push(new_value);
}