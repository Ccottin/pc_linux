use std::env;

#[derive(Debug)]
struct Term {
    coefficient: Option<f64>,
    exposant: Option<i32>, //if 0 == coeff * 1
    operator: Option<char>,
    sign: Option<char>,
}

impl Term {
    fn new() -> Term {
        Term {
            coefficient: None,
            exposant: None,
            operator: None,
            sign: None,
        }
    }

    fn erase(&mut self) {
        self.coefficient = None;
        self.exposant = None;
        self.operator = None;
    }
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
    arg = arg.trim_start();
    (0, arg)
}

fn  is_sign(c: char) -> bool {
    if c == '+' || c == '-' || c == '*' || c == '/' {
        return true
    }
    false
}

fn  is_numeric(c: u8) -> bool {
    if c == b'0' || c == b'1' || c == b'2' || c == b'3' || c == b'4'
        || c == b'5' || c == b'6' || c == b'7' || c == b'8' || c == b'9' {
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

fn  handle_digits(c: char, base: &[u8], index: usize, term: &mut Term)
    -> (isize, String) {
    match term.coefficient {
        Some(i) => {
             if base[index - 1] == b' ' && term.coefficient != None {
                return (-1, String::from("two consecutive numbers."))
            }
            else if base[index - 1] == b'.' {
                term.coefficient = Some(i + c.to_digit(10).unwrap() as f64 / 10.0);
            }  
            else {
                term.coefficient = Some(i * 10.0);
                term.coefficient = Some(i * c.to_digit(10).unwrap() as f64);
            }
        }
        None => term.coefficient = Some(c.to_digit(10).unwrap() as f64),
    }
    println!("{:?}", term.coefficient);
    (0, String::new())
}

fn next_char(base: &[u8], index: mut i32) -> u8 {
    while base[index] == b' ' {
        index = index + 1;
    }
    return base[index]
}

fn prev_char(base: &[u8], index: mut i32) -> u8 {
    while base[index] == b' ' {
        index = index - 1;
    }
    if index == 0 {
        return b'E'
    }
    return base[index]
}

fn  handle_signs(c: char, base: &[u8], index: &mut usize, term: &mut Term, nb_elem: isize)
    -> (isize, String) {
        let mut search = index + 1;

        if index == 0 && (c == '*' || c == '/') {
            return (-1, String::from("unvalid first operation."))
        }
        if (c == '-') {
            term.operator = Some('-');

        }

        // si on multiplie x, il ne se passe rien
        if c == '*' && is_numeric(prev_char(base, index)) && next_char(base, index) == b'x' {
            return (0, String::new())
        }
        if c == '/' && is_numeric(prev_char(base, index)) && is_numeric(next_char(base, index)) {
            if (term.coefficient != None) {
                term.coefficient = Some(term.coefficient / next_char(base, index).to_digit(10).unwrap() as f64)
                    //on en est là, push en PANIK
            }

    (0, String::new())
}

fn  handle_unknown(base: &[u8], index: usize)
    -> (isize, String) {
    let mut search = index - 1;
    while search >= 0 && base[search] == b' ' {
        search -= 1;
    }
    if is_sign(base[search] as char) || search == 0 || base[search] == b'=' {
        return (0, String::new())
    }
    (-1, String::from("Unvalid x syntax."))
}

fn  handle_power(base: &[u8], index: &mut usize, term: &mut Term)
    -> (isize, String) {
    let mut search = *index - 1;
    while search >= 0 && base[search] != b'x' && base[search] == b' ' {
        search -= 1;
    }
    if !base[search] == b'x' {
        return (-1, String::from("power symbol should be only after a x"));
    }
    search = *index + 1;
    while search <= base.len() && base[search] == b' ' {
        search += 1;
    }
    // if is_sign(base[search + 1] as char) || base[search + 1] != b' ' {
    //     return (-1, String::from("please provide a single number as power."));
    // }
    if is_numeric(base[search]) {
        let c1 = base[search] as char;
        term.exposant = Some(c1.to_digit(10).unwrap() as i32);
        *index = search + 1;
        return (0, String::new())
    }
    (-1, String::from("unvalid power operand."))
}


//IDS == en fonction du retour,deep copy to_fill dans un vec et 
//appel a la methode.erase
fn  parser(arg: String) -> (Vec<Term>, Vec<Term>) {


    let mut left_expression: Vec<Term> = Vec::new();
    let mut right_expression: Vec<Term> = Vec::new();
    let mut to_fill = Term::new();
    let mut ret = (0, String::new());
    let mut nb_elem = 0;
    
    for (mut i, item) in arg.char_indices() {
        
        if item.is_numeric() {
            println!("digit");
            ret = handle_digits(item, arg.as_bytes(), i, &mut to_fill)
        }
        else if is_sign(item) {
            println!("sign");
            ret = handle_signs(item, arg.as_bytes(), &mut i, &mut to_fill, nb_elem)
        }
        else if item == ' ' {
            println!("espasse");
            ret = (0, String::new());
        }
        else if item == 'x' {
            println!("x");
            ret = handle_unknown(arg.as_bytes(), i)
        }
        else if item == '=' {
            println!("=");
            if nb_elem == 0 {
                ret = (-1, String::from("no left-side term."));
            }
            ret.0 = 2;
        }
        else if item == '^' {
            println!("^");
            ret = handle_power(arg.as_bytes(), &mut i, &mut to_fill)
        }
        else if item == '.' {
            println!(".");
            if !(is_numeric(arg.as_bytes()[i - 1]) && is_numeric(arg.as_bytes()[i + 1])) {
                ret = (-1, String::from("unvalid dot."));
            }
        }
       
        else {
        println!("WTF");
        ret = (-1, String::from("WTF"));
        }

        println!("after loop {}, truct = {:?}", i, to_fill);
        if ret.0 == -1 {
            println!("error : {}", ret.1);
        }
        ret.0 = 0;
        //evaluer si ret = -1 ou 1
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
        return;
    }
    parser(arg.1);



    //potentially add a checker here
    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.

   // check_polynomial(&polynomial);

}
