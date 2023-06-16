pub fn special_pythagorean_triplet() -> u32 {
	let mut c;
	for a in 1..333 {
		for b in 1..499 {
			c = 1000 - a - b;
			if a*a + b*b == c*c {
				return a * b * c;
			}
		}
	}
	return 0;
}