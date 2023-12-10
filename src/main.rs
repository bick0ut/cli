use std::{fs, io};

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
            "help" => println!(
                "ls - list files in current directory\n\
                help - lists possible commands\n\
                exit - exits the program"
            ),
            "ls" => {
                let paths = fs::read_dir("./").unwrap();

                for path in paths {
                    let path = path.unwrap();
                    let md = path.metadata();
                    if md.unwrap().is_file(){
                        println!("{}", path.path().display().to_string().trim_start_matches("./"));
                    } else {
                        println!("/{}", path.path().display().to_string().trim_start_matches("./"));
                    }

                }
            },
            _n => println!("Unknown command {_n}")
        }

    }
}
