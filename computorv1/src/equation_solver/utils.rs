use crate::term::Term;

pub fn  add_missing_values(r_polynomial: &mut Vec<Term>) {
    let mut zero: bool = false;
    let mut one: bool = false;

    for item in &mut *r_polynomial {
        if item.exposant == 0 {
            zero = true;
        }
        if item.exposant == 1 {
            one = true;
        }
    }
    if !zero {
        r_polynomial.insert(0, Term::add(0));
    }
    if !one {
        r_polynomial.insert(1, Term::add(1));
    }
}

pub fn      first_degree(r_polynomial: Vec<Term>) -> f64 {
    let mut result: f64;

    if r_polynomial.len() == 2 {
        result = r_polynomial[0].coefficient * -1.0;
        result /= r_polynomial[1].coefficient;
        return result;
    }
    0.0
}