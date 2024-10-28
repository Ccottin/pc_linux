let degree = r_polynomial.last().unwrap().exposant;
    println!("Polynomial degree : {}", degree);
    if degree > 2 {
        println!("The polynomial degree is strictly greater than 2, I can't solve.");
        return false;
    }
    true