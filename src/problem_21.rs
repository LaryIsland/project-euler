pub fn amicable_numbers() -> u16 {
	let mut lookup_table: Vec<u16> = vec![];
	let mut answer: u16 = 0;
	let mut sum: u16;
	for i in 0..10001u16 {
		sum = 0;
		for j in 1..(i+1/2) {
			if i % j == 0 {
				sum += j;
			}
		}
		if sum > 10001 {
			sum = 0;
		}
		lookup_table.push(sum);
	}
	for i in 1..10001u16 {
		let tmp1 = lookup_table[i as usize];
		if i == lookup_table[tmp1 as usize] && i != tmp1 {
			answer += i;
		}
	}
	return answer;
}