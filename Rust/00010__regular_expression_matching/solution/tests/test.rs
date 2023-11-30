use solution::is_match;

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn matching_string_and_pattern_always_true(s in "[a-z]*") {
    	assert!(is_match(s.clone(), s))
    }

    #[test]
    fn nonmatching_string_and_pattern_always_false(s in "[a-z]*", p in "[a-z]*") {
    	prop_assume!(s != p);
    	assert!(!is_match(s, p))
    }

    #[test]
    fn basic_star(c in "[a-z]", n in 0..100) {
    	let s = c.repeat(n as usize);
    	let p = format!("{c}*");
    	println!("----> {}, {}", s, p);
    	assert!(is_match(s, p))
    }

	#[test]
    fn complex_star(c in "[a-z]", n in 1..100) {
    	let s = c.repeat(n as usize);
    	let p = format!("{c}*{c}");
    	assert!(is_match(s, p))
    }

	#[test]
    fn dot_star(c in "[a-z]", n in 1..100) {
    	let s = c.repeat(n as usize);
    	let p = format!(".*{c}");
    	assert!(is_match(s, p))
    }

	#[test]
    fn dot_equiv(c in "[a-z]", n in 1..100) {
    	let s = c.repeat(n as usize);
    	let p = ".".repeat(n as usize);
    	assert!(is_match(s, p))
    }

    #[test]
    fn star_for_empty_string(c in "[a-z]", n in 1..10) {
        let s = "".to_string();
        let p = format!("{c}*").repeat(n as usize);
        println!("{} :: {}", s, p);
        assert!(is_match(s, p))
    }
}