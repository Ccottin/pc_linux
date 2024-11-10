pub fn is_sign(c: u8) -> bool {
    if c == b'+' || c == b'-' || c == b'*' || c == b'/' {
        return true;
    }
    false
}

pub fn is_numeric(c: u8) -> bool {
    if c == b'0'
        || c == b'1'
        || c == b'2'
        || c == b'3'
        || c == b'4'
        || c == b'5'
        || c == b'6'
        || c == b'7'
        || c == b'8'
        || c == b'9'
    {
        return true;
    }
    false
}

pub fn next_char(base: &[u8], index: usize) -> u8 {
    if index == base.len() - 1 {
        return base[index];
    }

    let mut i = index + 1;
    while i < base.len() - 1 && base[i] == b' ' {
        i += 1;
    }
    base[i]
}

pub fn prev_char(base: &[u8], index: usize) -> u8 {
    if index == 0 {
        return base[index];
    }

    let mut i = index - 1;
    while i != 0 && base[i] == b' ' {
        i -= 1;
    }
    base[i]
}

pub fn get_nb_str(base: &[u8], index: &mut usize) -> (isize, String) {
    let mut ret = String::new();
    let mut dot: bool = false;

    while base[*index] == b' ' {
        *index += 1;
    }
    while *index < base.len() && (is_numeric(base[*index]) || base[*index] == b'.') {
        if base[*index] == b'.' {
            if *index + 1 < base.len() && is_numeric(base[*index + 1]) && !dot {
                dot = true;
            } else {
                return (-1, "please check your decimal numbers again.".to_string());
            }
        }
        ret.push(base[*index] as char);
        *index += 1;
    }
    if ret.len() > 15 {
        return (-1, "please provide fiveteen-digits numbers or else, otherwise precision will drop.".to_string());
    }
    *index -= 1;
    (0, ret)
}
