
use solution::{reverse, max_pow_10};

use proptest::prelude::*;
use proptest::strategy::{BoxedStrategy};

fn i32_natural() -> BoxedStrategy<i32> {
    prop_oneof![
    	0..10_000,
    	0..i32::MAX-10_000,
        0..i32::MAX
    ].boxed()
}

fn i32_fuzz() -> BoxedStrategy<i32> {
    prop_oneof![
    	i32::MIN..i32::MIN+500,
        i32::MAX-500..i32::MAX,
        -250..250,
        Just(i32::MIN),
        Just(i32::MAX),
        i32::MIN..i32::MAX
    ].boxed()
}


proptest! {
    #![proptest_config(ProptestConfig {
        cases: 100_000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn max_pow_10_matches_string_behaviour(n in i32_natural()) {
        let pow = max_pow_10(n) as usize;
    	assert_eq!(pow, n.to_string().chars().count()-1)
    }

	#[test]
    fn reverse_is_stable(n in i32_fuzz()) {
    	reverse(n);
    }

	#[test]
    fn reverse_on_positive_is_positive_or_zero(n in 0..=i32::MAX) {
    	assert!(0 <= reverse(n));
    }

	#[test]
    fn reverse_on_negative_is_negative_or_zero(n in i32::MIN..0i32) {
    	assert!(0 >= reverse(n));
    }

	#[test]
    fn reverse_matches_string_reverse_ignoring_huge_numbers(n in 0..147483647_i32) {
		let ours = reverse(n);
		let standard = n.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap();
		assert_eq!(ours, standard);

		let our_neg = reverse(-n);
		let standard_neg = -standard;

		assert_eq!(our_neg, standard_neg);
    }
}

#[test]
fn reverse_examples() {
	assert!(reverse(123) == 321);
	assert!(reverse(-123) == -321);
	assert!(reverse(120) == 21);

	assert!(reverse(i32::MAX) == 0);
	assert!(reverse(i32::MIN) == 0);
}