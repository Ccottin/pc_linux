use crate::term::Term;
use crate::print_results;


pub fn  use_quadratic_formula(a: f64, b: f64, c: f64) {
    let discriminant: f64;
    // D = b^2 -4ac
    discriminant = b * b - (4.0 * a * c);
    print_results::print_discriminant(discriminant);

    //we should print POLARITY of discriminant and solve according to it :) but
    //for now, its oc times
}