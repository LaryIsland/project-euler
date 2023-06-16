pub fn largest_prime_factor(mut number: u64) -> u64 {
	let mut i = 2;
	loop {
		if is_prime(i) && number % i == 0 {
			number /= i;
			if is_prime(number) {
				return number;
			}
		}
		else {
			i += 1
		}
	}
}

pub fn is_prime(n: u64) -> bool {
	if n == 2 || n == 3 { return true; }
	if n % 2 == 0 || n % 3 == 0 { return false; }

	let mut j = 5;
	while j * j <= n {
		if n % j == 0 || n % (j + 2) == 0 { return false; }
		j += 6
	}

	return true;
}