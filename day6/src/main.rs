use std::env;
use std::fs;
use std::thread::current;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Content: {content}");

    let starting_packet_index = find_pattern(&content, 4);

    println!("Starting packet at {}", starting_packet_index.unwrap());

    let starting_message_index = find_pattern(&content, 14);

    println!("Starting message at {}", starting_message_index.unwrap());
}

fn find_pattern(content: &String, chars_to_take: usize) -> Option<usize> {
    let mut starting_index: Option<usize> = None;
    let mut index = 0;

    while starting_index.is_none() {

        let current_slice = &content[index..index+chars_to_take];
        let mut chars = current_slice.chars();

        let mut temp_check = true;

        for i in 0..chars_to_take {
            for j in (i+1)..chars_to_take {
                //println!("i: {i}, j: {j}");
                //println!("Current slices: {} == {}", chars.clone().nth(i).unwrap(), chars.clone().nth(j).unwrap());
                // Porkaround: clone the list every time, since the nth consume the iterator...not really good for performances... I haven't still understood how to do it in other way
                if chars.clone().nth(i).unwrap() == chars.clone().nth(j).unwrap() {
                    temp_check = false;
                }
            }
        }

        if temp_check == true {
            starting_index = Some(index+chars_to_take);
        }

        index+=1;
    }

    return starting_index;
}