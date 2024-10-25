use std::env;

mod equation_solver;
mod input_handler;
mod term;

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
    let polynomial = input_handler::parser::parser(arg.1);
    if polynomial.0 == -1 {
        error(&polynomial.2);
        return ;
    }

    equation_solver::solve_equation(polynomial.1);

    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.
}
