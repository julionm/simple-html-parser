use parser::process;

mod parser;

fn main() {
    //let mut args = env::args();

    let html = "
    <html>
        <head>
            <style>
            </style>
        </head>
    <body>
        <div>
            <button>
            </button>
        </div>
        <button/>
        <input/>
    </body>
    </html>
    ";

    match process(html.into()) {
        Ok(_) => {
            println!("Everything went Ok!");
        }
        Err(err) => {
            println!("Error while trying to parse html!: {}", err);
        }
    }
}
