use divisors::get_divisors;

pub fn non_abundant_sums() -> u32 {
	let mut sum = 0;
	let mut summable = false;
	let abundant = abundant_num_generator();
	for i in 1..20162u32 {
		for num in &abundant {
			if num > &(i >> 1) {
				break;
			}
			if abundant.binary_search(&(i - num)).is_ok() {
				summable = true;
				break;
			}
		}
		if !summable {
			sum += i;
		}
		summable = false;
	}
	return sum;
}

fn abundant_num_generator() -> Vec<u32> {
	let mut abundant_nums: Vec<u32> = Vec::new();
	for i in 1..20162u32 {
		let sum: u32 = get_divisors(i).iter().sum();
		if sum > i {
			abundant_nums.push(i);
		}
	}
	return abundant_nums;
}