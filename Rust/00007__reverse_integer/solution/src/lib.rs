

pub fn reverse(x: i32) -> i32 {
    if x == i32::MIN { return 0 };

    match to_digits_reversed(x) {
        (Sign::Positive, arr) => to_num(arr),
        (Sign::Negative, arr) => -to_num(arr)
    }
}

pub enum Sign {
    Positive,
    Negative
}

pub fn to_digits_reversed(x: i32) -> (Sign, [u8; 10]) {
    let sign = if x < 0 { Sign::Negative } else { Sign::Positive };

    let mut n = i32::abs(x);
    let mut digits: [u8; 10] = [0; 10];

    let pow = max_pow_10(n);
    for i in 0..(pow+1) {
        digits[(9-(pow-i)) as usize] = (n % 10) as u8;
        n = n / 10;
    }
    (sign, digits)
}

pub fn to_num(digits: [u8; 10]) -> i32 {
    match digits[0] {
        d if d > 2 => 0,
        d if d < 2 => to_num_unsafe(&digits),
        2 =>  {
            let n = to_num_unsafe(&digits[1..]);
            if n > 147483647 { 0 } else { n + 2000000000 }
        }
        _ => unreachable!()
    }
}

pub fn to_num_unsafe(digits: &[u8]) -> i32 {
    let mut n = 0;
    let len = digits.len();
    for i in 0..len {
        n += (digits[(len-1)-i] as i32) * 10_i32.pow(i as u32)
    }
    n
}

pub fn max_pow_10(n: i32) -> i32 {
    const POWERS_OF_10: [i32; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
    
    if n <= 0 { return 0; }

    for i in 0..10 {
        if POWERS_OF_10[i] > n {
            return (i-1) as i32
        }
    }
    9
}
