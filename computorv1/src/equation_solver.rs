use crate::print_results;
use crate::term::Term;

mod equation_reducer;
mod quadratic_formula;

//a quick reminder : 34 * x ^ 0 = 34 * 1 = 34. The opposite works as well (;
// although, 34 * x ^ 1 = 34 * x

fn      first_degree(r_polynomial: Vec<Term>) -> f64 {
    let mut result: f64;

    if r_polynomial.len() == 2 {
        result = r_polynomial[0].coefficient * -1.0;
        result /= r_polynomial[1].coefficient;
        return result;
    }
    0.0
}

fn  add_missing_values(r_polynomial: &mut Vec<Term>) {
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

fn      is_infinite_results(r_polynomial: &Vec<Term>) -> bool {
    for item in r_polynomial.iter() {
        if item.coefficient != 0.0 {
            return false;
        }
    }
    print_results::infinite_solutions();
    true
}

pub fn  solve_equation(polynomial: [Vec<Term>; 2]) {
    let mut r_polynomial: Vec<Term>;
    let degree: isize;

    r_polynomial = equation_reducer::reduce_equation(polynomial);
    if r_polynomial.is_empty() {
        print_results::unvalid_equation();
        return ;
    }
    degree = r_polynomial.last().unwrap().exposant;
    print_results::print_polynomial_degree(degree);


    match degree {
        0 => print_results::one_solution(r_polynomial[0].coefficient),
        1 => {
            if !is_infinite_results(&r_polynomial) {
            print_results::one_solution(first_degree(r_polynomial));
            }
        }
        2 => {
            if r_polynomial.len() == 1 && r_polynomial[0].coefficient == 0.0 {
                print_results::infinite_solutions();
            }
            else if !is_infinite_results(&r_polynomial) {
                if r_polynomial.len() < 3 {
                    add_missing_values(&mut r_polynomial);
                }
                quadratic_formula::use_quadratic_formula(r_polynomial[2].coefficient, r_polynomial[1].coefficient, r_polynomial[0].coefficient);
            }
        }
        _ => (),
    }
}
