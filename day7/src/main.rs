use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::collections::HashMap;


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

    //let re = Regex::new("[ \\[\\]]*").unwrap();

    let mut current_directory: Vec<String> = Vec::new(); // in realtà questa è la current dir
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    let mut read_lines = buffered.lines();
    let mut lines: Vec<String> = Vec::new();
    for l in read_lines { // re-assign in a simple vector so that i can use peekable correctly after
        lines.push(l.unwrap());
    }

    let mut lines = lines.iter().peekable();

    while lines.peek().is_some() {
        let row = lines.next();
        let line = row.unwrap();
        println!("{}", line);

        // i suppose that the filesystem tree is navigated depth first, entering all the nodes in order and never turning back

        if line.starts_with("$ cd .."){

            let current_dir_string = current_directory.clone().join("/").replacen("//", "/", 1);
            let dir_size = dir_sizes.get(&current_dir_string).unwrap();
            current_directory.pop();

            let current_dir_string = current_directory.clone().join("/").replacen("//", "/", 1);
            let old_value = dir_sizes.get(&current_dir_string).unwrap();
            dir_sizes.insert(current_dir_string.clone(), dir_size + old_value); // ma guarda un po' se devo clonare il valore solo per poi stamparlo
            println!("{current_dir_string} New value = {}", dir_sizes.get(&current_dir_string).unwrap());

        } else if line.starts_with("$ cd "){
            let dir_name: String = line.split(' ').nth(2).unwrap().parse().unwrap(); // TODO another workaround, i still have to understand the best way to do this
            println!("Dir name: {dir_name}");
            current_directory.push(dir_name);
            let current_dir_string = current_directory.clone().join("/").replacen("//", "/", 1);
            println!("Current full path: {}", current_dir_string);
            dir_sizes.insert(current_dir_string, 0);
        } else if line == "$ ls" {

            loop {
                // first peek the element here
                let ls_line = lines.peek();
                if ls_line.is_some() {
                    if ls_line.unwrap().starts_with("$") {
                        break;
                    } else {
                        let item_line = lines.next().unwrap();
                        let current_dir_string = current_directory.clone().join("/").replacen("//", "/", 1);
                        println!("Item line: {item_line}, curr_dir: {current_dir_string}");

                        if item_line.starts_with("dir ") {
                            // ignore it, the navigation will happen anyway with cd <folder>
                        } else {
                            let file_size: usize = item_line.split(' ').nth(0).unwrap().parse().unwrap();
                            let mut value = dir_sizes.get_mut(&current_dir_string).unwrap();
                            *value = *value + file_size;
                            println!("{current_dir_string} New value = {}", *value);
                        }
                    }
                } else { // end file case
                    break;
                }

            }

        } else {
            // nothing to do
            println!("Warn: not a valid line");
        }
    }

    println!("Remaining stack: {}", current_directory.clone().join("/").replacen("//", "/", 1));
    loop {
        // equivalent of cd ..
        let current_dir_string = current_directory.clone().join("/").replacen("//", "/", 1);
        let dir_size = dir_sizes.get(&current_dir_string).unwrap();
        current_directory.pop();

        if current_directory.iter().peekable().peek().is_none() { // i do here the check, since the last element is /, and i can not pop anymore after it
            break;
        }

        let current_dir_string = current_directory.clone().join("/").replacen("//", "/", 1);
        let old_value = dir_sizes.get(&current_dir_string).unwrap();
        dir_sizes.insert(current_dir_string.clone(), dir_size + old_value); // ma guarda un po' se devo clonare il valore solo per poi stamparlo
        //let value = dir_sizes.get(&current_dir_string).unwrap();
        //*value = *value + *dir_size;
        println!("#{current_dir_string}# New value = {}", dir_sizes.get(&current_dir_string).unwrap());
    }

    // get all the dirs with total size <= 100k
    let mut little_dir_sizes = dir_sizes.clone();
    little_dir_sizes.retain(|_, v| *v <= 100000);
    let mut sum: usize = 0;
    for v in little_dir_sizes.values(){
        sum += *v;
    }
    println!("Total sum of dir under 100k: {sum}");

    let total_space: usize = 70000000;
    let min_space_to_free: usize = 30000000;

    let free_space = total_space - dir_sizes.get("/").unwrap();
    println!("Free space: {free_space}");

    let min_space_to_free = min_space_to_free - free_space;
    println!("Space to free: {min_space_to_free}");

    dir_sizes.retain(|_, v| *v > min_space_to_free);
    for (s, v) in &dir_sizes {
        println!("S: {s} {v}");
    }
    let max_min_dir = dir_sizes.iter().min_by_key(|entry | entry.1).unwrap();
    println!("Maximum minimum dir: {}, size: {}", max_min_dir.0, max_min_dir.1);
}
