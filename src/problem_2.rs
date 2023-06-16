pub fn even_fibonacci_numbers(limit: u64) -> u64 {
	let mut curr = 1;
	let mut prev = 1;
	let mut total = 0;
	while curr < limit {
		if curr % 2 == 0 {
			total += curr;
		}
		(prev, curr) = (curr, curr + prev);
	}
	return total;
}