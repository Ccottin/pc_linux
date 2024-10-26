use crate::term::Term;

//a quick reminder : 34 * x ^ 0 = 34 * 1 = 34. The opposite works as well (;

fn  search_exposant(to_search: Vec<Term>, to_find: isize) -> isize {
    for (i, item) in to_search.iter().enumerate() {
        if item.exposant == to_find {
            return itry_into().unwrap(); ;
        }
    }
    return -1;
}

fn      reduce_equation(polynomial: [Vec<Term>; 2]) {
    let mut reduced_form: Vec<Term>; 
    let mut to_push: Term;
    let mut size = 0;


    //first, lets reduce left-handed side.
    //we ned to copy vec into a temp bcs rust will not allow mutable and imuable ref at same time
    for (mut i, mut item) in polynomial[0].iter().enumerate() {
        let val = search_exposant(reduced_form, polynomial[0][i].exposant);
        if size > 0 && val > -1 {
            reduced_form[val].coefficient += polynomial[0][i].coefficient;
            reduced_form[val].sign *= sign;
        } else {
            to_push = item.clone();
            reduced_form.push(to_push);
        }
    
        // for part two

        // temp[i].coefficient *= temp[i].sign;
        // item.coefficient += temp[i].coefficient;
        // reduced_form.swap_remove(i);
        
    }

    println!("test one = {:?}", reduced_form);

    for item in polynomial[1].iter() {
        to_push = item.clone();
        to_push.sign *= -1.0;
        reduced_form.push(to_push);
    }
    println!("{:?}", reduced_form);
    //search for powers that are the same, so we can add them

}

pub fn  solve_equation(polynomial: [Vec<Term>; 2]) {
    println!("Starting to solve {:?} = {:?}", polynomial[0], polynomial[1]);
    reduce_equation(polynomial);
}
