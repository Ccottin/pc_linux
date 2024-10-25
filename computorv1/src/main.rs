use std::env;

mod input_handler;
mod term;
//a test si tu mets un mod term ici a la racine si tu peux y acceder dans input handler

fn error(reason: &str) {
    println!("Error: {reason}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error("Not enough arguments.");
        return;
    }

    let arg = input_handler::arg_checker::args_checker(&args[1..]);
    if arg.0 != 0 {
        error(&arg.1);
        return ;
    }
    let temp = input_handler::parser::parser(arg.1);
    if temp.0 == -1 {
        error(&temp.2);
        return ;
    }

    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.
}
