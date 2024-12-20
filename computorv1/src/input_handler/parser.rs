use crate::input_handler::utils::{get_nb_str, is_numeric, is_sign, next_char, prev_char};
use crate::term::MutTerm;
use crate::term::Term;


fn handle_digits(base: &[u8], index: &mut usize, term: &mut MutTerm) -> (isize, String) {
    //println!("char = {} ", base[*index] as char);
    match term.coefficient {
        Some(_) => return (-1, "too many coefficient for a term.".to_string()),
        None => {
            let tmp = get_nb_str(base, index);
            if tmp.0 == 0 {
                term.coefficient = Some(tmp.1.parse::<f64>().unwrap());
            } else {
                return tmp;
            }
        }
    }
    (0, "".to_string())
}

//------------------**************------------------//

fn handle_multiplication(base: &[u8], index: &mut usize, term: &mut MutTerm) -> (isize, String) {
    if (is_numeric(prev_char(base, *index)) && next_char(base, *index) == b'X')
        || (is_numeric(next_char(base, *index)) && prev_char(base, *index) == b'X')
    {
        return (0, String::new());
    } else if is_numeric(prev_char(base, *index)) && is_numeric(next_char(base, *index)) {
        *index += 1;

        let tmp = get_nb_str(base, index);
        if tmp.0 == -1 {
            return tmp;
        }

        if term.x && term.coefficient == None {
            term.coefficient = Some(1.0 * tmp.1.parse::<f64>().unwrap());
        } else {
            term.coefficient = Some(term.coefficient.unwrap() * tmp.1.parse::<f64>().unwrap());
        }
    } else {
        return (-1, "multiplication syntax error.".to_string());
    }
    (0, String::new())
}

fn handle_division(base: &[u8], index: &mut usize, term: &mut MutTerm) -> (isize, String) {
    if next_char(base, *index) == b'X' {
        return (
            -1,
            "division per x is forbidden".to_string(),
        );
    }
    if (is_numeric(prev_char(base, *index)) || prev_char(base, *index) == b'X')
        && is_numeric(next_char(base, *index))
    {
        *index += 1;

        let tmp = get_nb_str(base, index);
        if tmp.0 == -1 {
            return tmp;
        }

        let tmp2 = tmp.1.parse::<f64>().unwrap();
        if tmp2 == 0.0 {
            return (-1, "division per zero is forbidden.".to_string());
        }
        if term.x && term.coefficient == None {
            term.coefficient = Some(1.0 / tmp2);
        } else {
            term.coefficient = Some(term.coefficient.unwrap() / tmp2);
        }
    } else {
        return (-1, "division syntax error.".to_string());
    }
    (0, String::new())
}

fn handle_signs(c: u8, base: &[u8], index: &mut usize, term: &mut MutTerm) -> (isize, String) {
    if *index == 0 || prev_char(base, *index) == b'=' {
        match c {
            b'-' => {
                term.sign = Some(-1.0);
                return (0, String::new());
            }
            b'+' => {
                term.sign = Some(1.0);
                return (0, String::new());
            }
            _ => return (-1, "unvalid first operation.".to_string()),
        }
    }

    if (!is_numeric(prev_char(base, *index)) && prev_char(base, *index) != b'X')
        || *index + 1 == base.len()
    {
        println!("{}, {}", prev_char(base, *index) as char, base[*index] as char);
        return (-1, "sign syntax.".to_string());
    }

    match c {
        b'-' => return (2, String::new()),
        b'+' => return (1, String::new()),
        b'*' => return handle_multiplication(base, index, term),
        b'/' => return handle_division(base, index, term),
        _ => (),
    }
    (0, "".to_string())
}

//------------------**************------------------//

fn handle_unknown(term: &mut MutTerm, base: &[u8], index: usize) -> (isize, String) {
    if (index == 0 || is_sign(prev_char(base, index)) || prev_char(base, index) == b'=')
        && !term.x
        && (is_sign(next_char(base, index))
            || next_char(base, index) == b'='
            || next_char(base, index) == b'^'
            || index + 1 == base.len())
    {
        term.x = true;
        return (0, String::new());
    }
    (-1, "unvalid x syntax.".to_string())
}

//------------------**************------------------//

fn handle_power(base: &[u8], index: &mut usize, term: &mut MutTerm) -> (isize, String) {
    if prev_char(base, *index) != b'X' {
        return (-1, "power symbol should be only after a x".to_string());
    }
    if is_numeric(next_char(base, *index)) {
        while *index < base.len() - 1 && !is_numeric(base[*index]) {
            *index += 1;
        }
        let nb = get_nb_str(base, index);
        if nb.1.find('.') != None {
            return (-1, "powers should be whole numbers".to_string());
        }
        term.exposant = Some(nb.1.parse::<isize>().unwrap());
        return (0, String::new());
    }
    (-1, "unvalid power operand.".to_string())
}

//------------------**************------------------//

fn add_to_expression(expression: &mut Vec<Term>, term: &mut MutTerm) {
    if term.coefficient == None {
        term.coefficient = Some(1.0);
    }
    if term.sign != None {
        term.coefficient = Some(term.coefficient.unwrap() * term.sign.unwrap());
    }
    if term.exposant == None && term.x == false {
        term.exposant = Some(0);
    }
    else if term.exposant == None && term.x == true {
        term.exposant = Some(1);
    }
    expression.push(Term::new(term));
    term.erase();
}

pub fn parser(args: String) -> (isize, [Vec<Term>; 2], String) {
    let mut polynomial: [Vec<Term>; 2] = [Vec::new(), Vec::new()];
    let mut side = 0; //0 = left, 1 = right
    let mut term = MutTerm::new();
    let mut ret = (0, String::new());
    let mut i: usize = 0;
    let arg = args.as_bytes();

    while i < arg.len() {
        //maybe check l'overflow?
        if is_numeric(arg[i]) {
            ret = handle_digits(arg, &mut i, &mut term)
        } else if is_sign(arg[i]) {
            ret = handle_signs(arg[i], arg, &mut i, &mut term)
        } else if arg[i] == b'X' {
            ret = handle_unknown(&mut term, arg, i)
        } else if arg[i] == b'^' {
            ret = handle_power(arg, &mut i, &mut term)
        } else if arg[i] == b'.' {
            if i == 0 || !(is_numeric(arg[i - 1]) && is_numeric(arg[i + 1])) {
                ret = (-1, "please check your decimal numbers again.".to_string());
            }
        } else if arg[i] == b'=' {
            if term.coefficient == None && !term.x || (i + 1 == arg.len()) {
                ret = (-1, "syntax problem with '=' symbol.".to_string());
            } else {
                ret.0 = 3;
            }
        }

        match ret.0 {
            -1 => return (-1, polynomial, ret.1),
            1 => {
                add_to_expression(&mut polynomial[side], &mut term);
                term.sign = Some(1.0);
            }
            2 => {
                add_to_expression(&mut polynomial[side], &mut term);
                term.sign = Some(-1.0);
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
    (0, polynomial, "".to_string())
}
