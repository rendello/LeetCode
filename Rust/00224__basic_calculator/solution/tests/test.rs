use solution::{parse_number, compute_expression, calculate};

use proptest::prelude::*;
use proptest::strategy::{BoxedStrategy};


fn i32_natural() -> BoxedStrategy<i32> {
    prop_oneof![
    	0..500,
    	i32::MAX-500..i32::MAX,
        0..i32::MAX
    ].boxed()
}


proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn test_parse_number(n in i32_natural(), s in r"[a-zA-Z\(\) ]*") {
    	let input = format!("{n}{s}");
    	let (rest_slice, number) = parse_number(input.as_bytes());
    	let rest = std::str::from_utf8(rest_slice).to_owned().unwrap();

    	assert_eq!(rest, s);
    	assert_eq!(n, number);
    }

	#[test]
    fn test_compute_expression(n in i32_natural()) {
    	assert_eq!(compute_expression(format!("{n}").as_bytes()), n);
    }
}

#[test]
fn compute_expression_examples() {
	assert_eq!(2, calculate("1 + 1".to_string()));
	assert_eq!(3, calculate(" 2-1 + 2 ".to_string()));
	assert_eq!(23, calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
	assert_eq!(0, calculate("()".to_string()));
	assert_eq!(-123, calculate("(-123)".to_string()));
	assert_eq!(-143, calculate("((((-123))))-10+1+0+1+   1-12+(-1)".to_string()));
}