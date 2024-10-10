use std::env;

struct Term {
    coefficient: i32,
    exposant: i32,
    operator: [char; 2],
}

fn error(reason: &str) {
    println!("Error: {reason}");
}

fn check_polynomial(polynomial: &String) {
    return;
}

fn args_checker(args: &Vec::<String>) -> String {
    let mut equal = 0;

    for s in args {

        println!("{s}");
        let byte = s.as_bytes();

        for (i, &item) in byte.iter().skip(1).enumerate() {
            println!("for {i} arg is {item}");
        }
    }
    "".to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error("Not enough arguments.");
        return;
    }
    if args_checker(&args).is_empty()  {
        error("Unvalid format");
      //  return;
    }

    let test = Term {
        coefficient : 2,
        exposant : 1,
        operator : ['-', '+'],
    };

    println!("{0}, {1}, {2}", test.coefficient, test.exposant, test.operator[1]);
    //potentially add a checker here
    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.

   // check_polynomial(&polynomial);

    dbg!(args);
}
