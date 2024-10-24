fn is_valid_char(arg: &String) -> bool {
    let valid_chars: [u8; 9] =
        [b'.', b'+', b'-', b'*', b'=', b'x', b'^', b' ', b'/'];

    for b in arg.as_bytes() {
        if !valid_chars.contains(b) && !b.is_ascii_digit() {        // digit => 0..9
            return false
        }
    }
    true
}

pub fn args_checker(args: &[String]) -> (i32, String) {

    let mut equal = 0;
    let mut arg = String::new();

    // get a string with all input strings
    for s in args {
        arg = arg + s;
    }
    if !arg.is_ascii() || !is_valid_char(&arg) {
        return (1, "Unvalid set of caracters.".to_string())
    }

    let c = arg.as_bytes();

    for (_i, &item) in c.iter().enumerate() {
        if item == b'=' {
            equal += 1;
        }
    }
    if equal != 1 {
        return (1, "Unvalid equality sign.".to_string())
    }
    arg = String::from(arg.trim_start());
    arg = String::from(arg.trim_end());
    (0, arg)
}
