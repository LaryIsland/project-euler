pub fn summation_of_primes() -> u64 {
	let mut prime_total = 2u64;
	let mut i = 3u32;
	while i < 2_000_000 {
		if is_prime(i) {
			prime_total += i as u64;
		}
		i += 2;
	}
	return prime_total;
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