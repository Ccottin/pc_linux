use crate::term::Term;
use crate::print_results;

// we use Heron algorithm ; xn+1 = 1/2 * (xn + S/xn)
// precision is insuficient for large number, so we have to cut at 16 numbers
//it is not done yet, think about a neet way to handle this
fn      square_root(s: f64) -> f64 {
    let mut x = s;
    let mut x1;
    let trunc: isize;
    
    loop {
        x1 = 0.5 * (x + (s / x));

        if x1 == x {

            // x *= 10e9;
            // trunc = x as isize;
            // x = trunc as f64;
            // x /= 10e9;
            return x;
        }
        x = x1;
    }
}

// quadratic formula gives us 2 answer according to discriminant : 
pub fn  use_quadratic_formula(a: f64, b: f64, c: f64) {
    let mut res: (f64, f64) = (0.0, 0.0);

    // D = b^2 -4ac
    let discriminant = b * b - (4.0 * a * c);
    println!("disc = {}, squrt = {} == {}", discriminant, square_root(a), a.sqrt());
    //uncomment to test square root fn
    if discriminant < 0.0 {
        print_results::print_discriminant(-1);
        return ;
    }

    let disc_sqrt = square_root(discriminant);
    if discriminant == 0.0 {
        print_results::print_discriminant(0);
        res.0 = (-b + disc_sqrt) / (2.0 * a);
        print_results::double_root(res.0);
    }
    else {
        print_results::print_discriminant(1);
        res.0 = (-b + disc_sqrt) / (2.0 * a);
        res.1 = (-b - disc_sqrt) / (2.0 * a);
        print_results::two_solutions(res);
    }
}