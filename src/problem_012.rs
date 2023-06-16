pub fn highly_divisible_triangular_number() -> u32 {
	let primes = primes_to_limit(6173);
	let mut number = 0;
	let mut i = 0;
	let mut max;
	let mut power = 0;
	let mut tmp;
	loop {
		i += 1;
		number += i;
		max = 1;
		tmp = number;
		for prime in primes.iter() {
			while tmp % prime == 0 {
				tmp /= prime;
				power += 1;
			}
			if power != 0 {
				max *= power + 1;
				power = 0;
				if tmp == 1 { break; }
				if is_prime(tmp) {
					max *= 2;
					break;
				}
			}
		}
		if max > 500 {
			return number;
		}
	}
}

fn primes_to_limit(limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![];
    for i in 2..limit+1 {
        if is_prime(i) {
            primes.push(i);
		}
	}
	return primes;
}

pub fn is_prime(n: u32) -> bool {
	if n == 2 || n == 3 { return true; }
	if n % 2 == 0 || n % 3 == 0 { return false; }

	let mut j = 5;
	while j * j <= n {
		if n % j == 0 || n % (j + 2) == 0 { return false; }
		j += 6
	}

	return true;
}