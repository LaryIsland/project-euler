pub fn smallest_multiple(limit: u8) -> u64 {
	let mut result = 1;
	let primes = primes_to_limit(limit);
	let mut i;
    for prime in primes {
		i = 1;
		while prime.checked_pow(i) != None && prime.checked_pow(i).unwrap() < limit {
			result *= prime as u64;
			i += 1;
		}
	}
    return result
}

fn primes_to_limit(limit: u8) -> Vec<u8> {
    let mut primes: Vec<u8> = vec![];
    for i in 2u8..limit+1 {
        if is_prime(i) {
            primes.push(i)
		}
	}
	return primes;
}

pub fn is_prime(n: u8) -> bool {
	if n == 2 || n == 3 { return true; }
	if n % 2 == 0 || n % 3 == 0 { return false; }

	let mut j = 5;
	while j * j <= n {
		if n % j == 0 || n % (j + 2) == 0 { return false; }
		j += 6
	}

	return true;
}