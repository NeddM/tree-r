use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let counter: usize = 0;

    if args.contains(&"-h".to_string()) {
        print_help();
    } else if args.len() > 1 {
        treeing(&args[1], counter)
    } else {
        treeing(".", counter)
    }
}

fn treeing(new_dir: &str, mut counter: usize) {
    match fs::read_dir(new_dir) {
        Ok(data) => {
            for entry in data {
                match entry {
                    Ok(entry) => {
                        let spaces: String = "      ".repeat(counter);
                        if entry.path().is_dir() {
                            println!("{}{}", spaces, entry.file_name().to_str().unwrap());

                            counter += 1;
                            treeing(entry.path().to_str().unwrap(), counter);
                            counter -= 1;
                        } else if entry.path().is_file() {
                            println!("{}{}", spaces, entry.file_name().to_str().unwrap());
                        }
                    }
                    Err(e) => println!("Error code: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Folder not found");
            println!("Please enter a valid directory");
            println!("Error code: {}", e);
            return;
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("This script allows you to print every file and folder next by your current path.");
    println!("You can tree any folder by adding the path as a first argument.");
}
