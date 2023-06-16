pub fn largest_palindrome_product(digits: u32) -> u64 {
	let mut largest: u64 = 0;
	let mut temp: u64;
	for x in (1*10u64.pow(digits-1)..1*10u64.pow(digits)).rev() {
		if largest > x * 999 { break; }
		for y in (x..1*10u64.pow(digits)).rev() {
			temp = x * y;
			if temp == reverse(temp) && temp > largest {
				largest = temp;
			}
		}
	}
	return largest
}

fn reverse(mut number: u64) -> u64 {
	let mut reversed = 0;
	while number != 0 {
		reversed = reversed * 10 + number % 10;
		number /= 10
	}
	return reversed;
}