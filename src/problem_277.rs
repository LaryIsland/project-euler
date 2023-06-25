const SEQUENCE_OF_REMAINDERS: [u8; 30] = [1,0,0,0,1,2,2,2,0,0,1,0,0,2,2,0,2,0,2,2,0,0,1,0,0,2,1,1,0,2];

pub fn a_modified_collatz_sequence() -> u64 {
	let mut starting_point: u64 = 0;
	let mut increment: u64 = 1;
	let mut counter_max: u8 = 1;
	let mut result_one: u64 = 0;

	while counter_max < SEQUENCE_OF_REMAINDERS.len() as u8 {
		result_one = collatz_to_max(starting_point, counter_max, increment);
		starting_point = result_one + increment;
		let result_two = collatz_to_max(starting_point + increment, counter_max, increment);
		increment = result_two - result_one;
		starting_point = result_one;
		counter_max += 1;
	}
	starting_point = (1_000_000_000_000_000 / increment + 1) * increment + result_one;
	return collatz_to_max(starting_point, counter_max, increment);
}

fn collatz_to_max(starting_point: u64, counter_max: u8, increment: u64) -> u64 {
	let mut i: u64 = starting_point;
	let mut counter: u8;
	let mut collatz_num: u64;
	let mut curr_remainder: u8;
	loop {
		collatz_num = i;
		counter = 0;
		for next_remainder in SEQUENCE_OF_REMAINDERS{
			match collatz_num % 3 {
				0 => {collatz_num /= 3; curr_remainder = 0},
				1 => {collatz_num = (4 * collatz_num + 2) / 3; curr_remainder = 1},
				2 => {collatz_num = (2 * collatz_num - 1) / 3; curr_remainder = 2},
				_ => {print!("Error"); return u64::MAX}
			}
			if curr_remainder != next_remainder {
				break;
			}
			counter += 1;
			if counter >= counter_max {
				return i;
			}
		}
		i += increment;
	}
}