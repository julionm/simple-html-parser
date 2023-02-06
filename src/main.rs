mod parser;
mod utils;

use std::{env, fs::File, io::Read};
use parser::process;

fn main() {
    let mut args = env::args();
    args.next();

    let next_arg = args.next();

    match next_arg {
        Some(path) => {
            let file = File::open(path);

            let mut file = match file {
                Ok(f) => f,
                Err(err) => panic!("Error while opening the file: {}", err)
            };

            let mut html: String = String::new();

            match file.read_to_string(&mut html) {
                Ok(_) => (),
                Err(err) => panic!("I/O Error on file reading: {}", err)
            }

            match process(html) {
                Ok(_) => (),
                Err(err) => panic!("Error while trying to parse html!: {}", err)
            }
        },
        None => help()
    }

}

fn help() {
    println!("Pass a valid file path like: app-name \"tmp/index.html\"");
}
