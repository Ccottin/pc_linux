use crate::term::Term;

pub fn  unvalid_equation() {
    println!("equation is unvalid, I can't solve.");
}

pub fn  print_reduced_form(reduced_form: &Vec<Term>) {
    let mut reduced_string = String::new();

    for (i, item) in reduced_form.iter().enumerate() {
        if i == 0 {
            reduced_string = format!("{}", item.coefficient.to_string());
            }
        else if i > 0 && item.coefficient < 0.0 {
            reduced_string = format!("{reduced_string} - {}", (item.coefficient * -1.0).to_string());
        }
       
        else {
            reduced_string = format!("{reduced_string} + {}", item.coefficient.to_string());
        }
        reduced_string = format!("{reduced_string} * X^{}", item.exposant.to_string());
    }
    println!("Reduced form : {} = 0", reduced_string);
}

pub fn  print_polynomial_degree(degree: isize) {
    println!("Polynomial degree : {}", degree);
    if degree > 2 {
        println!("The polynomial degree is strictly greater than 2, I can't solve.");
    }
}

fn      truncate_nb(nb: f64) -> String {
    let mut ret = nb.to_string();

    if nb < 0.0 {
        if ret.len() <= 16 {
            return ret;
        }
        ret.truncate(16);
        ret += "[...]";
    } else {
        if ret.len() <= 15 {
            return ret;
        }
        ret.truncate(15);
        ret += "[...]";
    }
    ret
}

pub fn  reals_solutions(solution: (f64, f64)) {
    let to_print0 = truncate_nb(solution.0);
    let to_print1 = truncate_nb(solution.1);
    
    if solution.0 == solution.1 {
        println!("{}", to_print0);
    } else {
        println!("{}\n{}", to_print0, to_print1);
    }
}

pub fn  unreals_solutions(solution: (String, String)) {
    println!("{}\n{}", solution.0, solution.1);
}

pub fn  one_solution(solution: f64) {
    let to_print0 = truncate_nb(solution);

    println!("The solution is:\n{}", to_print0);
}

pub fn  print_discriminant(discriminant: isize) {
    if discriminant < 0 {
        println!("Discriminant is strictly negative, no real solution found. Complex solutions are:");
    }
    else if discriminant == 0 {
        println!("Discriminant is null, the solution (double root) is :");
    }
    else {
        println!("Discriminant is strictly positive, the two solutions are:");
    }
}

pub fn  infinite_solutions() {
    println!("all real solutions are possible, results interval:\n(+∞. -∞)")
}