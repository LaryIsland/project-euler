pub fn nth_prime(n: u32) -> u32 {
	let mut prime_counter = 1;
	let mut i = 3;
	loop {
		if is_prime(i) {
			prime_counter += 1;
			if prime_counter == n {
				return i;
			}
		}
		i += 2;

	}
}

fn is_prime(n: u32) -> bool {
	if n == 2 || n == 3 { return true; }
	if n % 2 == 0 || n % 3 == 0 { return false; }

	let mut j = 5;
	while j * j <= n {
		if n % j == 0 || n % (j + 2) == 0 { return false; }
		j += 6
	}

	return true;
}