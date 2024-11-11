use crate::print_results;
use crate::term::Term;

//function returns index of the elem with matching exposant, or -1 if its not here yet
fn  search_exposant(to_search: &Vec<Term>, to_find: isize) -> isize {
    for (i, item) in to_search.iter().enumerate() {
        if item.exposant == to_find {
            return i as isize; 
        }
    }
    -1
}

//search for powers that are the same, so we can add them
fn  reduce_side(side: &Vec<Term>, reduced_form: &mut Vec<Term>) {
    let mut to_push: Term;
    let mut size = reduced_form.len();

    for (i, item) in side.iter().enumerate() {
        let index = search_exposant(&reduced_form, side[i].exposant);
        if size > 0 && index > -1 {
            reduced_form[index as usize].coefficient += side[i].coefficient;
        } 
        else {
            to_push = item.clone();
            reduced_form.push(to_push);
            size = size + 1;
        }
    }
}

fn  merge_sides(left_side: &mut Vec<Term>, right_side: &Vec<Term>) {
    let mut to_push: Term;
    let mut size = left_side.len();
    
    for (i, item) in right_side.iter().enumerate() {
        let index = search_exposant(&left_side, right_side[i].exposant);
        if size > 0 && index > -1 {
            left_side[index as usize].coefficient -= right_side[i].coefficient;
        //    if left_side[index as usize].coefficient == 0.0 {
          //      left_side.swap_remove(index as usize);
            //}
        } else { // if item.coefficient != 0.0 {
            to_push = item.clone();
            to_push.coefficient *= -1.0;
            left_side.push(to_push);
            size = size + 1;
        }
        
    }
}

fn  got_x_forreal(side: &Vec<Term>) -> bool {
    let mut index = search_exposant(&side, 1);

    if index > -1 && side[index as usize].coefficient != 0.0 {
        return true;
    }
    index = search_exposant(&side, 2);
    if index > -1 && side[index as usize].coefficient != 0.0 {
        return true;
    }
    false
}

fn  find_side_const(side: &Vec<Term>) -> f64 {
    let index = search_exposant(&side, 0);

    if index == -1 {
        return 0.0;
    }
    side[index as usize].coefficient
}

fn  is_solvable(left_side: &Vec<Term>, right_side: &Vec<Term>) -> bool {
    if !got_x_forreal(&left_side) && !got_x_forreal(&right_side) {
        let ls_const = find_side_const(&left_side);
        let rs_const = find_side_const(&right_side);
        if ls_const != rs_const {
            return false
        }
    }
    true
}

pub fn  reduce_equation(polynomial: [Vec<Term>; 2]) -> Vec<Term> {
    let mut left_reduced_form: Vec<Term> = Vec::new();
    let mut right_reduced_form: Vec<Term> = Vec::new();
    let mut i = 0;
    let mut y = 0;

    //should find a solution ; the test must be exercised on a reduced shit, so we might
    //divide and reduce ; ls and rs should be already reduced so we can compare, find
    //if thers no x , and if theres not ; eval equality, and... well. and wat

    reduce_side(&polynomial[0], &mut left_reduced_form);
    reduce_side(&polynomial[1], &mut right_reduced_form);
    if !is_solvable(&left_reduced_form, &right_reduced_form) {
        return Vec::new();
    }
    println!("before merge side = {:?}, {:?}", left_reduced_form, right_reduced_form);
    merge_sides(&mut left_reduced_form, &right_reduced_form);
    println!("after merge side = {:?} ", left_reduced_form);
   
    //we will use a while loop, since a for loop would use immutable ref to reduced_form
    while y < left_reduced_form.len() {
        if left_reduced_form[i].exposant < left_reduced_form[y].exposant {
            left_reduced_form.swap(i, y);
        }
        i += 1;
        if i == left_reduced_form.len() {
            y += 1;
            i = y;
        }
    }
    print_results::print_reduced_form(&left_reduced_form);
    left_reduced_form
}