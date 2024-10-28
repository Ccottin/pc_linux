use crate::term::Term;

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

pub fn  one_solution(solution: f64) {
    println!("The solution is:\n{}", solution.to_string());
}

pub fn  print_discriminant(discriminant: f64) {
    //to late
}
