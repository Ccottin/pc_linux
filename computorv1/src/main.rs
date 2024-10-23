use std::env;

#[derive(Debug)]
struct MutTerm {
    coefficient: Option<f64>,
    exposant: Option<isize>, //if 0 == coeff * 1
    sign: Option<char>,
    x: bool,
}

impl MutTerm {
    fn new() -> MutTerm {
        MutTerm {
            coefficient: None,
            exposant: None,
            sign: None,
            x: false,
        }
    }

    fn erase(&mut self) {
        self.coefficient = None;
        self.exposant = None;
        self.sign = None;
        self.x = false;
    }
}

#[derive(Debug)]
struct Term {
    coefficient: f64,
    exposant: isize, //if 0 == coeff * 1
    sign: char,
    x: bool,
}

impl Term {
    fn new(origin: &mut MutTerm) -> Term {
        Term {
            coefficient: origin.coefficient.unwrap(),
            exposant: origin.exposant.unwrap(),
            sign: origin.sign.unwrap(),
            x: origin.x,
        }
    }
}

fn error(reason: &str) {
    println!("Error: {reason}");
}

fn is_valid_char(arg: &String) -> bool {
    let valid_chars: [u8; 9] =
        [b'.', b'+', b'-', b'*', b'=', b'x', b'^', b' ', b'/'];

    for b in arg.as_bytes() {
        if !valid_chars.contains(b) && !b.is_ascii_digit() {        // digit => 0..9
            return false
        }
    }
    true
}

fn  is_sign(c: u8) -> bool {
    if c == b'+' || c == b'-' || c == b'*' || c == b'/' {
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

fn args_checker(args: &[String]) -> (i32, String) {

    let mut equal = 0;
    let mut arg = String::new();

    // get a string with all input strings
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
    arg = String::from(arg.trim_start());
    (0, arg)
}


fn  get_nb_str(base: &[u8], index: &mut usize) -> String {
    let mut ret = String::new();
    let mut dot: bool = false;

    while base[*index] == b' ' {
        *index += 1;
    }
    while is_numeric(base[*index]) && *index < base.len() {
        ret.push(base[*index] as char);
        *index = *index + 1;
        if *index == base.len() {
            *index = *index - 1;
            return ret
        }
        if base[*index] == b'.' && dot == false {
            ret.push(base[*index] as char);
            *index = *index + 1;
            dot = true;
        }
    }
    *index = *index - 1;
    ret
}

fn  handle_digits(base: &[u8], index: &mut usize, term: &mut MutTerm)
    -> (isize, String) {
    println!("char = {} ", base[*index] as char);
    match term.coefficient {
        Some(_) => return (-1, String::from("too many coefficient for a term")),
        None => term.coefficient = Some(get_nb_str(base, index).parse::<f64>().unwrap()),
    }
    (0, String::new())
}

fn next_char(base: &[u8], index: usize) -> u8 {
    if index == base.len() {
        return base[index]
    }
    let mut i = index + 1;
    while base[i] == b' ' && i < base.len() {
        i = i + 1;
    }
    base[i]
}

fn prev_char(base: &[u8], index: usize) -> u8 {
    if index == 0 {
        return base[index]
    }
    let mut i = index - 1;
    while base[i] == b' ' && i != 0 {
        i = i - 1;
    }
    base[i]
}

fn  handle_signs(c: u8, base: &[u8], index: &mut usize, term: &mut MutTerm)
    -> (isize, String) {
    if *index == 0 || prev_char(base, *index) == b'=' {
        match c {
            b'-' => {
                term.sign = Some('-');
                return (0, String::new())
            }
            b'+' => {
                term.sign = Some('+');
                return (0, String::new())
            }
            _ => return (-1, String::from("unvalid first operation.")),
        }
    }

    match c {
        b'-' => return (2, String::new()),
        b'+' => return (1, String::new()),
        b'*' => {
            if (is_numeric(prev_char(base, *index)) && next_char(base, *index) == b'x') ||
                (is_numeric(next_char(base, *index)) && prev_char(base, *index) == b'x') {
                return (0, String::new())
            }
            else if is_numeric(prev_char(base, *index)) && is_numeric(next_char(base, *index)) {
                *index = *index + 1;
                term.coefficient = Some(term.coefficient.unwrap()
                    * get_nb_str(base, index).parse::<f64>().unwrap());
            }
            else {
                return (-1, String::from("please provide mutiplication with x or numbers only."))
            }
        }
        b'/' => {
            if is_numeric(prev_char(base, *index)) && is_numeric(next_char(base, *index)) {
                if term.coefficient != None {
                    *index = *index + 1;
                    let temp = get_nb_str(base, index).parse::<f64>().unwrap();
                    if temp == 0.0 {
                        return (-1, String::from("division per zero is forbidden."))
                    }
                    term.coefficient = Some(term.coefficient.unwrap() / temp);
                }
            }
            else {
                return (-1, String::from("please provide divisions with numbers only."))
            }
        }
        other => println!("What? I just got {}", other),
    }
    (0, String::new())
}

fn  handle_unknown(term: &mut MutTerm, base: &[u8], index: usize)
    -> (isize, String) {
    if (index == 0 || is_sign(prev_char(base, index)) || prev_char(base, index) == b'=')
        && term.x == false {
        term.x = true;
        return (0, String::new())
    }
    (-1, String::from("unvalid x syntax."))
}

fn  handle_power(base: &[u8], index: &mut usize, term: &mut MutTerm)
    -> (isize, String) {
    if prev_char(base, *index) != b'x' {
        return (-1, String::from("power symbol should be only after a x"));
    }
    if is_numeric(next_char(base, *index)) {
        while !is_numeric(base[*index]) {
            *index += 1;
        }
        let nb = get_nb_str(base, index);
        if nb.find('.') != None {
            return (-1, "powers should be whole numbers".to_string())
        }
        term.exposant = Some(nb.parse::<isize>().unwrap());
        return (0, String::new())
    }
    (-1, String::from("unvalid power operand."))
}

fn  add_to_expression(expression: &mut Vec<Term>, term: &mut MutTerm) {
    if term.sign == None {
        term.sign = Some('+');
    }
    if term.coefficient == None {
        term.coefficient = Some(1.0); 
    }
    if term.exposant == None {
        term.exposant = Some(0);
    }
    expression.push(Term::new(term));
    term.erase();
}

fn  parser(args: String) -> (isize, [Vec<Term>; 2]) {

    let mut polynomial: [Vec<Term>; 2] = [Vec::new(), Vec::new()];
    let mut side = 0;   //0 = left, 1 = right
    let mut term = MutTerm::new();
    let mut ret = (0, String::new());
    let mut i: usize = 0;
    let mut item: u8;   
    let arg = args.as_bytes();

    while i < arg.len() {
        item = arg[i];
        if is_numeric(item) {
            ret = handle_digits(arg, &mut i, &mut term)
        }
        else if is_sign(item) {
            ret = handle_signs(item, arg, &mut i, &mut term)
        }
        else if item == b' ' {
            ret = (0, String::new());
        }
        else if item == b'x' {
            ret = handle_unknown(&mut term, arg, i)
        }
        else if item == b'=' {
            if term.coefficient == None && term.x == false {
                ret = (-1, String::from("problem with left-side term."));
            }
            else {
                ret.0 = 3;
            }
        }
        else if item == b'^' {
            ret = handle_power(arg, &mut i, &mut term)
        }
        else if item == b'.' {
            if i == 0 || !(is_numeric(arg[i - 1]) && is_numeric(arg[i + 1])) {
                ret = (-1, String::from("unvalid dot."));
            }
        }
        else {
        println!("WTF");
        ret = (-1, String::from("WTF"));
        }

        println!("after loop {}, struct = {:?} \n", i, term);
        match ret.0 {
            -1 => {
                println!("error : {}", ret.1);
                return (-1, polynomial)
            }
            1 => {
                add_to_expression(&mut polynomial[side], &mut term);
                term.sign = Some('+');
            }
            2 => {
                add_to_expression(&mut polynomial[side], &mut term);
                term.sign = Some('-');
            }
            3 => {
                add_to_expression(&mut polynomial[side], &mut term);
                side = 1;
            }
            _ => (),
        }
        ret.0 = 0;
        i += 1;
    }
    add_to_expression(&mut polynomial[side], &mut term);
    println!("{:?} \n=\n {:?}", polynomial[0], polynomial[1]);
    (0, polynomial)
}

// *------------------------------* //

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error("Not enough arguments.");
        return;
    }

    let arg = args_checker(&args[1..]);
    if arg.0 != 0 {
        error(&arg.1);
        return ()
    }
    if parser(arg.1).0 == -1 {
        return ()
    }



    //potentially add a checker here
    //Note that std::env::args will panic if any argument contains
    //invalid Unicode.

   // check_polynomial(&polynomial);

}
