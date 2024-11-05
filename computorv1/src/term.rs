#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub exposant: isize, //if 0 == coeff * 1
}

impl Term {
    pub fn new(origin: &mut MutTerm) -> Term {
        Term {
            coefficient: origin.coefficient.unwrap(),
            exposant: origin.exposant.unwrap(),
        }
    }
    pub fn add(exposant: isize) -> Term {
        Term {
            coefficient: 0.0,
            exposant: exposant,
        }
    }
}

#[derive(Debug)]
pub struct MutTerm {
    pub coefficient: Option<f64>,
    pub exposant: Option<isize>, //if 0 == coeff * 1
    pub sign: Option<f64>,
    pub x: bool,
}

impl MutTerm {
    pub fn new() -> MutTerm {
        MutTerm {
            coefficient: None,
            exposant: None,
            sign: None,
            x: false,
        }
    }

    pub fn erase(&mut self) {
        self.coefficient = None;
        self.exposant = None;
        self.sign = None;
        self.x = false;
    }
}
