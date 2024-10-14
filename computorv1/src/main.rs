use std::env;

struct Term {
    coefficient: f64,
    exposant: i32, //if 0 == coeff * 1
    //operator: char,
    sign: char,

}

fn error(reason: &str) {
    println!("Error: {reason}");
}

fn is_valid_char(arg: &String) -> bool {
    let valid_chars: [u8; 7] = [b'.', b'+', b'-', b'*', b'=', b'x', b'^'];

    for b in arg.as_bytes() {
        if !valid_chars.contains(&b) && !b.is_ascii_digit() {
            return false
        }
    }
    true
    // We just did this and theres a quote issue
}

fn args_checker(args: &[String]) -> (i32, String) {
    
    let mut equal = 0;
    let mut arg = String::new();

    for s in args {
        arg = arg + s
    }
    if !arg.is_ascii() || !is_valid_char(&arg) {
        return (1, "Unvalid set of caracters".to_string())
    }

    println!("{arg}");
    let c = arg.as_bytes();

    for (i, &item) in c.iter().enumerate() {
        println!("for {i} arg is {item}");
        if item == b'=' {
            equal += 1;
        }

    }

    println!("{equal}");
    (0, "".to_string())
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

    //potentially add a checker here
    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.

   // check_polynomial(&polynomial);

}
