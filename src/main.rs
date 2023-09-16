use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let counter: usize = 0;

    if args.contains(&"-h".to_string()) {
        print_help();
    } else if args.len() > 1 {
        iteration(&args[1], counter)
    } else {
        iteration(".", counter)
    }
}

fn iteration(new_dir: &str, mut counter: usize) {
    match fs::read_dir(new_dir) {
        Ok(_) => {}
        Err(_) => {
            println!("Folder not found");
            println!("Please enter a valid directory");
            return;
        }
    }

    for entry in fs::read_dir(new_dir).unwrap() {
        match entry {
            Ok(entry) => {
                let spaces: String = "      ".repeat(counter);
                if entry.path().is_dir() {
                    println!("{}{}", spaces, entry.file_name().to_str().unwrap());

                    counter += 1;
                    iteration(entry.path().to_str().unwrap(), counter);
                    counter -= 1;
                } else if entry.path().is_file() {
                    println!("{}{}", spaces, entry.file_name().to_str().unwrap());
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("This script allows you to print every file and folder next by your current path.");
    println!("You can tree any folder by adding the path as a first argument.");
}
