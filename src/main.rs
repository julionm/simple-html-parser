use parser::process;

mod parser;

fn main() {
    //let mut args = env::args();

    let html = String::from("<html>
    <head>
        <style>
        </style>
    </head>
    <body>
        <div>
            <button>
            </button>
        </div>
        <button>
        </button>
    </body>
</html>
    ");

    match process(html) {
        Ok(_) => {
            println!("Everything went Ok!");
        }
        Err(err) => {
            println!("Error while trying to parse html!: {}", err);
        }
    }
}
