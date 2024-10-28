use crate::print_results;
use crate::term::Term;

//maybe I should do a bonus file since this is ok for mandatory part but not bonus
fn  search_exposant(to_search: &Vec<Term>, to_find: isize) -> isize {
    for (i, item) in to_search.iter().enumerate() {
        if item.exposant == to_find {
            return i as isize; 
        }
    }
    return -1;
}

//search for powers that are the same, so we can add them
fn  reduce_equation_1(polynomial: &Vec<Term>, reduced_form: &mut Vec<Term>, left_side: bool) {
    let mut to_push: Term;
    let mut size = reduced_form.len();

    for (i, item) in polynomial.iter().enumerate() {
        let val = search_exposant(&reduced_form, polynomial[i].exposant);
        if size > 0 && val > -1 && left_side {
            reduced_form[val as usize].coefficient += polynomial[i].coefficient;
        } 
        else if size > 0 && val > -1 && !left_side {
            reduced_form[val as usize].coefficient -= polynomial[i].coefficient;
        }
        else {
            to_push = item.clone();
            if !left_side && polynomial[i].coefficient != 0.0 {
                to_push.coefficient *= -1.0;
            }
            reduced_form.push(to_push);
            size = size + 1;
        }
    }
}

pub fn  reduce_equation(polynomial: [Vec<Term>; 2]) -> Vec<Term> {
    let mut reduced_form: Vec<Term> = Vec::new();
    let mut i = 0;
    let mut y = 0;
   
    reduce_equation_1(&polynomial[0], &mut reduced_form, true);
    reduce_equation_1(&polynomial[1], &mut reduced_form, false);
   
    //we will use a while loop, since a for loop would use immutable ref to reduced_form
    while y < reduced_form.len() {
        if reduced_form[i].exposant < reduced_form[y].exposant {
            reduced_form.swap(i, y);
        }
        i += 1;
        if i == reduced_form.len() {
            y += 1;
            i = y;
        }
    }
    print_results::print_reduced_form(&reduced_form);
    reduced_form
}