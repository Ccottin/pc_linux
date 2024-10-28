use crate::print_results;
use crate::term::Term;

mod equation_reducer;
mod quadratic_formula;
mod polynomial_degree;

//should i do a mod just for outputs?
//a quick reminder : 34 * x ^ 0 = 34 * 1 = 34. The opposite works as well (;
// although, 34 * x ^ 1 = 34 * x



pub fn  solve_equation(polynomial: [Vec<Term>; 2]) {
    let mut r_polynomial: Vec<Term>;
    let discriminant: f64;

    println!("Starting to solve {:?} = {:?}", polynomial[0], polynomial[1]);
    r_polynomial = equation_reducer::reduce_equation(polynomial);
    if !print_results::eval_print_polynomial_degree(&r_polynomial) {
        return ;
    }
    if find_polynomial_degree >
    quadratic_formula::use_quadratic_formula(r_polynomial);
}
