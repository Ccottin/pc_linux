use std::env;

#[derive(Debug)]
struct Term {
    coefficient: Option<f64>,
    exposant: Option<i32>, //if 0 == coeff * 1
  //  operator: Option<char>,
    sign: Option<char>,
    x: bool,
}

impl Term {
    fn new() -> Term {
        Term {
            coefficient: None,
            exposant: None,
          //  operator: None,
            sign: None,
            x: false,
        }
    }

    fn erase(&mut self) {
        self.coefficient = None;
        self.exposant = None;
      //  self.operator = None;
        self.sign = None;
        self.x = false;
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
    arg = String::from(arg.trim_start());
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

fn  get_nb_str(base: &[u8], index: &mut usize) -> String {
    let mut ret = String::new();
    let mut dot: bool = false;

    while is_numeric(base[*index]) {
        ret.push(base[*index] as char);
        *index = *index + 1;
        if base[*index] == b'.' && dot == false {
            ret.push(base[*index] as char);
            *index = *index + 1;
            dot = true;
        }
    }
    ret
}

fn  handle_digits(base: &[u8], index: &mut usize, term: &mut Term)
    -> (isize, String) {
    match term.coefficient {
        Some(_) => return (-1, String::from("too many coefficient for a term")),
        None => term.coefficient = Some(get_nb_str(base, index).parse::<f64>().unwrap()),
    }
    println!("{:?}", term.coefficient);
    (0, String::new())
}

fn next_char(base: &[u8], index: usize) -> u8 {
    let mut i = index;
    while base[i] == b' ' {
        i = i + 1;
    }
    return base[i]
}

fn prev_char(base: &[u8], index: usize) -> u8 {
    let mut i = index;
    while base[i] == b' ' {
        i = i - 1;
    }
    if i == 0 {
        return b'E'
    }
    return base[i]
}

fn  handle_signs(c: char, base: &[u8], index: &mut usize, term: &mut Term, nb_elem: isize)
    -> (isize, String) {
    if *index == 0 && (c == '*' || c == '/') {
        return (-1, String::from("unvalid first operation."))
    }

    match c {
        '-' => return (2, String::new()),
        '+' => return (1, String::new()),
        '*' => {
            // si on multiplie x, il ne se passe rien
            if is_numeric(prev_char(base, *index)) && next_char(base, *index) == b'x' ||
                is_numeric(next_char(base, *index)) && prev_char(base, *index) == b'x'{
                return (0, String::new())
            }
            else if is_numeric(prev_char(base, *index)) && is_numeric(next_char(base, *index)) {
                *index = *index + 1;
                term.coefficient = Some(term.coefficient.unwrap()
                    * get_nb_str(base, index).parse::<f64>().unwrap());
            }
            else {
                return (-1, String::from("Please provide mutiplication with x or numbers only."))
            }
        }
        '/' => {
            if is_numeric(prev_char(base, *index)) && is_numeric(next_char(base, *index)) {
                if term.coefficient != None {
                    *index = *index + 1;
                    term.coefficient = Some(term.coefficient.unwrap()
                        / get_nb_str(base, index).parse::<f64>().unwrap());
                }
            }
            else {
                return (-1, String::from("Please provide divisions with numbers only."))
            }
        }
        other => println!("What? I just got {}", other),
    }
    (0, String::new())
}

fn  handle_unknown(term: &mut Term, base: &[u8], index: usize)
    -> (isize, String) {
    let mut search = index - 1;
    while search >= 0 && base[search] == b' ' {
        search -= 1;
    }
    if is_sign(base[search] as char) || search == 0 || base[search] == b'=' {
        return (0, String::new())
    }
    if term.x == false {
        term.x == true;
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

fn  add_to_expression(expression: &mut Vec<Term>, to_fill: &mut Term, nb_elem: &mut isize) {
    if to_fill.sign == None {
        to_fill.sign = Some('+');
    }
    if to_fill.coefficient == None {
        to_fill.coefficient = Some(1.0); 
    }
    if to_fill.exposant == None {
        to_fill.exposant = Some(0);
    }
    expression.push(to_fill);
    to_fill.erase();
    *nb_elem = *nb_elem + 1;
}

//IDS == en fonction du retour,deep copy to_fill dans un vec et 
//appel a la methode.erase
fn  parser(arg: String) -> (isize, (Vec<Term>, Vec<Term>)) {

    let mut polynomial: (Vec<Term> , Vec<Term>) = (Vec::new(), Vec::new());
    let mut side = 0;   //0 = left, 1 = right // this dont work, jjust go eat
    let mut to_fill = Term::new();
    let mut ret = (0, String::new());
    let mut nb_elem = 0;
    
    for (mut i, item) in arg.char_indices() {
        
        if item.is_numeric() {
            println!("digit");
            ret = handle_digits(arg.as_bytes(), &mut i, &mut to_fill)
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
            ret = handle_unknown(&mut to_fill, arg.as_bytes(), i)
        }
        else if item == '=' {
            println!("=");
            if nb_elem == 0 {
                ret = (-1, String::from("no left-side term."));
            }
            ret.0 = 3;
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
        //evaluer si ret = -1 ou 1 ou 2 ou 3

        println!("after loop {}, truct = {:?}", i, to_fill);
        match ret.0 {
            -1 => {
                println!("error : {}", ret.1);
                return (-1, polynomial)
            }
            1 => {
                polynomial.side.push(add_to_expression(&mut to_fill));
                to_fill.sign = Some('+');
            }
            2 => {
                polynomial.side.push(add_to_expression(&mut to_fill));
                to_fill.sign = Some('-');
            }
            3 => side = 1,
            _ => (),
        }
        ret.0 = 0;
    }
    println!("{:?} = {:?}", polynomial.0, polynomial.1);
    (0, polynomial)
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
