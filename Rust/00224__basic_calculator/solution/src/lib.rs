const I32_POWERS: usize = 10;

enum State {
    Add,
    Sub
}

pub fn calculate(s: String) -> i32 {
    compute_expression(s.as_bytes())
}


fn get_char(stream: &[u8]) -> Option<u8> {
    if !stream.is_empty() { Some(stream[0]) } else { None }
}

fn digit(stream: &[u8]) -> Option<u8> {
    match get_char(stream) {
        Some(c) if c >= 48 && c <= 57 => Some(c-48),
        _ => None
    }
}

fn digits_to_i32(acc: [u8; I32_POWERS], depth: usize) -> i32 {
    let mut n: i32 = 0;
    for i in 0..depth {
        n += acc[i] as i32 * 10_i32.pow((depth - 1 - i) as u32);
    }
    n
}

pub fn parse_number(stream: &[u8]) -> (&[u8], i32) {
    let acc: [u8; 10] = [0; 10];
    parse_number_i(stream, 0, acc)
}

fn parse_number_i(stream: &[u8], depth: usize, mut acc: [u8; I32_POWERS])
    -> (&[u8], i32) {
    match digit(stream) {
        Some(n) => {
            acc[depth] = n;
            parse_number_i(&stream[1..], depth+1, acc)
        }
        None => (stream, digits_to_i32(acc, depth))
    }
}

pub fn compute_expression(stream: &[u8]) -> i32 {
    let (_, n) = compute_expression_i(stream, 0, State::Add);
    n
}

fn compute_expression_i(stream: &[u8], n: i32, state: State) -> (&[u8], i32) {
    match get_char(stream) {
        Some(32) => compute_expression_i(&stream[1..], n, state),
        Some(43) => compute_expression_i(&stream[1..], n, State::Add),
        Some(45) => compute_expression_i(&stream[1..], n, State::Sub),
        Some(c) if c >= 48 && c <= 57 => {
            let (rest, parsed_n) = parse_number(&stream);
            let new_n = match state { State::Add => n+parsed_n, State::Sub => n-parsed_n };
            compute_expression_i(rest, new_n, State::Add)
        },
        Some(40) => {
            let (rest, group_n) = compute_expression_i(&stream[1..], 0, State::Add);
            let new_n = match state { State::Add => n+group_n, State::Sub => n-group_n };
            compute_expression_i(rest, new_n, State::Add)
        },
        Some(41) => (&stream[1..], n),
        None => (stream, n),
        _ => unreachable!()
    }
}