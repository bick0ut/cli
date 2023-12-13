use std::{fs, io};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The command to perform
    command: Option<String>,

    /// The pattern to look for
    pattern: Option<String>,

    /// The path of the file to read
    path: Option<std::path::PathBuf>
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let command = args.command.unwrap_or_else(|| String::from("loop"));
    let pattern = args.pattern.unwrap_or_else(|| String::from(""));
    let path = args.path.unwrap_or_else(|| std::path::PathBuf::from(""));


    match command.as_str() {
        "loop" => input_loop(),
        "grep" => {
            if pattern.is_empty() || path.display().to_string().is_empty() {
                return Err(CustomError("Please enter a valid pattern and path".to_string()))
            }
            return grep(&pattern, path);
        },
        _n => println!("Unknown command {}", _n)
    }

    Ok(())
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    //let result = fs::read_to_string(&args.path).expect("could not read file");

    // match result {
    //     Ok(content) => { println!("File content: {}", content); }
    //     Err(error) => { println!("Oh noes: {}", error); }
    // }



}
fn input_loop() {
    let mut input = String::new();
    loop {
        input.clear();
        println!("\nPlease enter a command (type help for list of commands):");
        io::stdin().read_line(&mut input).unwrap();
        // For manually handling cases
        // match io::stdin().read_line(&mut input) {
        //     Ok(n) => {}
        //     Err(error) => println!("error: {}", error)
        // }
        match input.as_str().trim() {
            "exit" => break,
            "help" => print_help(),
            "ls" => print_ls(),
            _n => println!("Unknown command {_n}")
        }
    }
}

fn grep(pattern: &String, path: std::path::PathBuf) -> Result<(), CustomError>{
    // consider using https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html
    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn print_help() {
    println!(
        "ls - list files in current directory\n\
                help - lists possible commands\n\
                exit - exits the program"
    )
}

fn print_ls() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path = path.unwrap();
        let md = path.metadata();
        if md.unwrap().is_file() {
            println!("{}", path.path().display().to_string().trim_start_matches("./"));
        } else {
            println!("/{}", path.path().display().to_string().trim_start_matches("./"));
        }
    }
}
