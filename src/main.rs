use std::{fs, io};
use clap::Parser;

struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
fn main() {
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

fn match_command(string: String){

}
fn print_help(){
    println!(
        "ls - list files in current directory\n\
                help - lists possible commands\n\
                exit - exits the program"
    )
}

fn print_ls(){
    {
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
}