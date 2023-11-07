
fn main() {
    println!("{}", reverse(-2147483322));
    // println!("{}", max_power_of_10(100));
}

pub fn reverse(n: i32) -> i32 {
    let mut current_n = n;
    let mut digits: [u8; 10] = [0; 10];
    let negative: bool = {
        if n == i32::MIN {
            return 0
        } else if n < 0 { 
            current_n = current_n * -1;
            true
        } else {
            false
        }
    };
    let mpow = max_power_of_10(current_n);

    for i in 0..mpow {
        digits[(mpow-1 - i) as usize] = (current_n % 10) as u8;
        current_n = current_n / 10;
    }
    f(&digits, negative)
}

fn to_num(digits: &[u8]) -> i32 {
    let mut n: i32 = 0;
    let len = digits.len();
    for i in 0..len {
        n += (digits[len-1-i] as i32) * 10_i32.pow((len-1-i) as u32)
    }
    n
}

fn f(digits: &[u8; 10], negative: bool) -> i32 {
    let res = match digits[0] {
        d if d > 2 => 0,
        d if d < 2 => to_num(digits),
        2 => {
            let n = to_num(&digits[1..]);
            println!("{:?}", digits);
            println!("{:?}", &digits[1..]);
            println!("{}", n);
            if !negative && n > 147483647 {
                0
            } else if negative && n > 147483648 {
                0
            } else {
                n + 2000_000_000
            }
        }
        _ => unreachable!()
    };
    if negative { -res } else { res }
}

fn max_power_of_10(n: i32) -> i32 {
    const POWERS_OF_10: [i32; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
    for i in 0i32..10 {
        if POWERS_OF_10[i as usize] > n {
            return i
        }
    }
    10
}