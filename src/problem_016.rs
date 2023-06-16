use num_bigint::BigUint;

pub fn power_digit_sum() -> BigUint {
	let mut num = BigUint::from(2u8).pow(1000);
	let mut sum = BigUint::from(0u8);
	loop {
		sum += &num % 10u8;
		num /= BigUint::from(10u8);
		if num <= BigUint::from(0u8) {
			break;
		}
	}
	return sum;
}