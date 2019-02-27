mod lexer;
mod lisp;
mod parser;
mod built_in;
#[cfg(feature = "web-spa")]
mod browser;

use std::fs;
use std::io::{self, BufRead, Write};

#[cfg(feature = "web-spa")]
fn main() {
    let mut context = built_in::init_context();

    let bytes = include_bytes!("../res/init.lisp");
    let init_str = String::from_utf8_lossy(bytes).to_string();

    lisp::eval_in_context(&init_str, &mut context);

    use stdweb::web::*;
    use yew::prelude::*;
    use yew::services::console::ConsoleService;
    use yew::services::storage::{Area, StorageService};

    yew::initialize();

    let app: App<_, browser::RootModel> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

#[cfg(not(feature = "web-spa"))]
fn main() {
    let mut context = built_in::init_context();

    let bytes = include_bytes!("../res/init.lisp");
    let init_str = String::from_utf8_lossy(bytes).to_string();
    lisp::eval_in_context(&init_str, &mut context);

    let stdin = io::stdin();
    print(">> ");
    
    for line in stdin.lock().lines() {
        let r = lisp::eval_in_context(&String::from(line.unwrap()), &mut context);
        println!("<< {:?}", r);
        print(">> ");
    }
}

fn print(s: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(&s.as_bytes()).expect("Cannot write to stdout");
    handle.flush().expect("Cannot write to stdout");
}