mod arg_checker;
mod parser;

#[derive(Debug)]
struct MutTerm {
    coefficient: Option<f64>,
    exposant: Option<isize>, //if 0 == coeff * 1
    sign: Option<char>,
    x: bool,
}

impl MutTerm {
    fn new() -> MutTerm {
        MutTerm {
            coefficient: None,
            exposant: None,
            sign: None,
            x: false,
        }
    }

    fn erase(&mut self) {
        self.coefficient = None;
        self.exposant = None;
        self.sign = None;
        self.x = false;
    }
}

#[derive(Debug)]
struct Term {
    coefficient: f64,
    exposant: isize, //if 0 == coeff * 1
    sign: char,
    x: bool,
}

impl Term {
    fn new(origin: &mut MutTerm) -> Term {
        Term {
            coefficient: origin.coefficient.unwrap(),
            exposant: origin.exposant.unwrap(),
            sign: origin.sign.unwrap(),
            x: origin.x,
        }
    }
}