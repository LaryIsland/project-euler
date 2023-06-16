pub fn longest_collatz_sequence() -> u32{
	let mut max: (u32, u16) = (0, 0);
	let mut counter: u16;
	let mut value: u32;

	for i in (500_001..1_000_000).step_by(2).rev() {
		counter = 0;
		value = i;

		loop {
			if value == 16 {
				counter += 3;
				break;
			}
			if value % 2 == 0 {
				value /= 2;
				counter += 1;
			} else {
				value = 3 * value + 1;
				counter += 1;
			}
		}
		if counter > max.1 {
			max.0 = i;
			max.1 = counter;
		}
	}
	return max.0;
}