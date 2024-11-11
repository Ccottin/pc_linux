use crate::print_results;

// we use Heron algorithm ; xn+1 = 1/2 * (xn + S/xn)
fn      square_root(s: f64) -> f64 {
    let mut x = s;
    let mut x1;
    let mut trunc: String;
    
    if s == 0.0 {
        return s;
    }
    loop {
        x1 = 0.5 * (x + (s / x));
        if x1 == x {
            trunc = x1.to_string();
            if trunc.len() > 15 {
                trunc.truncate(15);
                x = trunc.parse().unwrap();
            }
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
    //juste pour enlever le - devant le zero 
    if res.0 == -0.0 {
        res.0 = 0.0 }; 
    if res.1 == -0.0 {
        res.1 = 0.0 };
    print_results::reals_solutions(res);
}

// quadratic formula gives us 2 answer according to discriminant wich is :
// D = b^2 -4ac
// then answers are 
// res = (-b +-(sqrt(D)) / 2 * a

pub fn  use_quadratic_formula(a: f64, b: f64, c: f64) {

    let discriminant = b * b - (4.0 * a * c);
    if discriminant < 0.0 {
        negative_discriminant(a, b, discriminant);
    } else {
        positive_discriminant(a, b, discriminant);
    }
}