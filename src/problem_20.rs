use num_bigint::BigUint;

pub fn factorial_digit_sum(num: u8) -> u32 {
	
	let mut calc = factorial(BigUint::from(num));
	let mut sum: BigUint = BigUint::from(0u8);

	loop {
		sum += &calc % 10u8;
		calc /= BigUint::from(10u8);
		if calc <= BigUint::from(0u8) {
			break;
		}
	}
	return sum.to_u32_digits()[0];
}



fn factorial(num: BigUint) -> BigUint {
	let one: BigUint = BigUint::from(1u8);
	if num == one {
		return one;
	}
	let tmp = factorial(&num - one);
	return num * tmp;
}