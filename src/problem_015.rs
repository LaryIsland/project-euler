use num_bigint::BigUint;

pub fn lattice_paths(grid_size: u8) -> u64 {
	return (factorial(2*grid_size) / factorial(grid_size).pow(2)).iter_u64_digits().fold(0, |acc, digit| (acc << 1) + digit);
}

fn factorial(n: u8) -> BigUint {
	let mut answer = BigUint::from(1u8);
	for i in 1..n+1 {
		answer *= BigUint::from(i);
    }
	return answer;
}