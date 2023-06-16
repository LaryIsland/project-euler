use num_bigint::BigUint;

pub fn one_thousand_digit_fibonacci_number() -> u16 {
	let mut curr: BigUint = BigUint::from(1u8);
	let mut prev: BigUint = BigUint::from(1u8);
	let mut i: u16 = 3;
	loop {
		let temp = prev + &curr;
		prev = curr;
		curr = temp;
		if curr.to_string().chars().count() >= 1000 {
			return i;
		}
		i += 1;
	}
}