use std::io::Result;

use parser::process;

mod parser;

#[tokio::main]
async fn main() {
    //let mut args = env::args();

    let body = reqwest::get("https://www.wikipedia.org").await;

    match body {
        Ok(res) => {
            let text = res.text().await;

            match text {
                Ok(v) => println!("{}", v),
                Err(err) => panic!("{}", err)
            } 
        },
        Err(err) => panic!("{}", err)
    }

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
