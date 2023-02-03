use std::env;

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

    let (rest, html_as_nodes) = process(html);
    
    println!("the rest: {}", rest);
}
