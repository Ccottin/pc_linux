use std::env;

struct Term {
    coefficient: f64,
    exposant: i32, //if 0 == coeff * 1
    operator: char,
    sign: char,

}

fn error(reason: &str) {
    println!("Error: {reason}");
}

fn is_valid_char(arg: &String) -> bool {
    let valid_chars: [u8; 9] =
        [b'.', b'+', b'-', b'*', b'=', b'x', b'^', b' ', b'/'];

    for b in arg.as_bytes() {
        if !valid_chars.contains(&b) && !b.is_ascii_digit() {
            return false
        }
    }
    true
}

fn args_checker(args: &[String]) -> (i32, String) {

    let mut equal = 0;
    let mut arg = String::new();

    for s in args {
        arg = arg + s;
    }
    if !arg.is_ascii() || !is_valid_char(&arg) {
        return (1, "Unvalid set of caracters".to_string())
    }

    let c = arg.as_bytes();

    for (_i, &item) in c.iter().enumerate() {
        if item == b'=' {
            equal += 1;
        }
    }
    if equal != 1 {
        return (1, "Unvalid equality sign".to_string())
    }
    (0, arg)
}

fn  is_sign(c: char) -> bool {
    if c == '+' || c == '-' || c == '*' || c == '/' {
        return true
    }
    false
}
#[derive(Debug)]
enum Possibilities {
    Digit(char),                   // ("0123456789")
    Signs(char),                   // ("+-*/")
    WhiteSpace(char),              // (' ')
    Unknown(char),                 // ('x')
    Power(char),                   // ('^')
    Equal(char),                   // ('=')
    Dot(char),                     // ('.')
}

fn  check_possibilities(checked: Possibilities, base: &String, index: usize, term: &Term)
    -> (isize, &Term) {
    match checked {
        Possibilities::Digit(c) => {
            println!("{:?}", checked);

        }
        Possibilities::Signs(c) => {

        }
        Possibilities::WhiteSpace(c) => {

        }
        Possibilities::Unknown(c) => {

        }
        Possibilities::Power(c) => {

        }
        Possibilities::Equal(c) => {

        }
        Possibilities::Dot(c) => {

        }
    }
}

fn  parser(arg: String) -> (Vec<Term>, Vec<Term>) {


    let mut left_expression: Vec<Term> = Vec::new();
    let mut right_expression: Vec<Term> = Vec::new();


    for (i, item) in arg.char_indices() {
        
        if is_sign(item) {
            println!("sign");
            check_possibilities(Possibilities::Signs(item), &arg, i);
        }
        else if item.is_numeric() {
            println!("digit");
            check_possibilities(Possibilities::Digit(item), &arg, i);
        }
        else if item == ' ' {
            println!("espasse");
            check_possibilities(Possibilities::WhiteSpace(item), &arg, i);
        }
        else if item == 'x' {
            println!("x");
            check_possibilities(Possibilities::Unknown(item), &arg, i);
        }
        else if item == '=' {
            println!("=");
            check_possibilities(Possibilities::Equal(item), &arg, i);
        }
        else if item == '.' {
            println!(".");
            check_possibilities(Possibilities::Dot(item), &arg, i);
        }
        else if item == '^' {
            println!("^");
            check_possibilities(Possibilities::Power(item), &arg, i);
        }
        else {
        println!("WTF");
        }
    }
    (left_expression, right_expression)
}

// *------------------------------* //

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error("Not enough arguments.");
        return;
    }

    // let test = Term {
    //     coefficient : 2,
    //     exposant : 1,
    //     sign : '-',
    // };
  //  println!("{0}, {1}, {2}", test.coefficient, test.exposant, test.sign[1]);

    let arg = args_checker(&args[1..]);
    if arg.0 != 0 {
        error(&arg.1);
    }
    parser(arg.1);



    //potentially add a checker here
    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.

   // check_polynomial(&polynomial);

}
