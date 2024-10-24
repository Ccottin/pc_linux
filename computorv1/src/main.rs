use std::env;

mod input_handler;
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

    let arg = input_handler::args_checker(&args[1..]);
    if arg.0 != 0 {
        error(&arg.1);
        return ()
    }
    if input_handler::parser::parser(arg.1).0 == -1 {
        error(arg.2);
        return ()
    }

    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.
}
