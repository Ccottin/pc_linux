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

            //why not use a string to cut it to a certain lenght? if theres a dot
            // x *= 10e9;
            // trunc = x as isize;
            // x = trunc as f64;
            // x /= 10e9;
            return x;
        }
        x = x1;
    }
}

fn      negative_discriminant(a: f64, b: f64, discriminant: f64) {
    let mut res: (String, String) = (String::new(), String::new());
    let ir_disc = discriminant * -1.0;

    print_results::print_discriminant(-1);
    let disc_sqrt = square_root(ir_disc);
    let div = 2.0 * a;
    res.0 = format!("{} + {} * i", -b / div, disc_sqrt / div);
    res.1 = format!("{} - {} * i", -b / div, disc_sqrt / div);
    print_results::unreals_solutions(res);
}

fn      positive_discriminant(a: f64, b: f64, discriminant: f64) {
    let mut res: (f64, f64) = (0.0, 0.0);
    let disc_sqrt = square_root(discriminant);

    if discriminant == 0.0 {
            print_results::print_discriminant(0);
    } else {
        print_results::print_discriminant(1);
    }
    res.0 = (-b + disc_sqrt) / (2.0 * a);
    res.1 = (-b - disc_sqrt) / (2.0 * a);
    print_results::reals_solutions(res);
}

// quadratic formula gives us 2 answer according to discriminant : 
pub fn  use_quadratic_formula(a: f64, b: f64, c: f64) {

    // D = b^2 -4ac
    let discriminant = b * b - (4.0 * a * c);
    // println!("disc = {}, squrt = {} == {}", discriminant, square_root(a), a.sqrt());
    //uncomment to test square root fn
    // res = (-b +-(sqrt(D)) / 2 * a
    if discriminant < 0.0 {
        negative_discriminant(a, b, discriminant);
    } else {
        positive_discriminant(a, b, discriminant);
    }
}