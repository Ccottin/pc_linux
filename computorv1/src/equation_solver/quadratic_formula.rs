use crate::term::Term;
use crate::print_results;

// we use Heron algorithm ; xn+1 = 1/2 * (xn + S/xn)
fn      square_root(s: f64) -> f64 {
    let mut x = s;
    let mut x1;
    let mut i = 0;
    
    loop {
        println!("{}", x);
        x1 = 0.5 * (x + (s / x));

        if x1 == x {
            println!("{}", x1);
            return x1;
        }
        x = x1;
        i += 1;   
    }
    println!("final = {}", x);
    x
}

// quadratic formula gives us 2 answer according to discriminant : 
pub fn  use_quadratic_formula(a: f64, b: f64, c: f64) {
    let discriminant: f64;
    let res: (f64, f64);
    // D = b^2 -4ac
    discriminant = b * b - (4.0 * a * c);

    println!("disc = {}, squrt = {} == {}", discriminant, square_root(a), a.sqrt());
    // if discriminant < 0 
    //     print_results::print_discriminant(-1);
    // }
    // else if discriminant == 0 {
    //     print_results::print_discriminant(0);
    // }
    // else {
    //     print_results::print_discriminant(1);

    // }
}