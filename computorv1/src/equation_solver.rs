use crate::term::Term;

//a quick reminder : 34 * x ^ 0 = 34 * 1 = 34. The opposite works as well (;



fn      reduce_equation(polynomial: [Vec<Term>; 2]) {
    let mut reduced_form: Vec<Term> = polynomial[0].to_vec();
    let mut to_push: Term;

    //first, lets reduce left-handed side.
    //we ned to copy vec into a temp bcs rust will not allow mutable and imuable ref at same time
    //fuck it does not work
    //hellllllllllllllx
    for (mut i, mut item) in reduced_form.iter_mut().enumerate() {
        let find = i + 1;
        let mut temp = reduced_form.clone();
        while find < reduced_form.len() {
            if item.exposant == temp[i].exposant {
                temp[i].coefficient *= temp[i].sign;
                item.coefficient += temp[i].coefficient;
                reduced_form.swap_remove(i);
            } else {
                i += 1;
            }
            
        }
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
