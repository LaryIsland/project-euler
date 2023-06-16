pub fn lexicographic_permutations(position: u32) -> u32 {
	let mut n: u32 = 10;
	let mut combinations: u32 = 3628800; // n!/(n - r)! == n!
	let mut num: u32;
	let mut lower_bound: u32 = 0;
	let mut answer: Vec<u32> = Vec::new();
	let mut remaining: Vec<u32> = vec![0,1,2,3,4,5,6,7,8,9];

	while n > 0 {
		combinations /= n;
		num = 0;
		while lower_bound + combinations < position {
			lower_bound += combinations;
			num += 1;
		}
		answer.push(remaining.remove(num as usize));
		n -= 1;
	}
	return answer.iter().fold(0, |acc, elem| acc * 10 + elem);
}