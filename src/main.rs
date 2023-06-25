use std::io::Write;
use std::collections::HashMap;

mod problem_001;
mod problem_002;
mod problem_003;
mod problem_004;
mod problem_005;
mod problem_006;
mod problem_007;
mod problem_008;
mod problem_009;
mod problem_010;
mod problem_011;
mod problem_012;
mod problem_013;
mod problem_014;
mod problem_015;
mod problem_016;
mod problem_017;
mod problem_018;
mod problem_019;
mod problem_020;
mod problem_021;
mod problem_022;
mod problem_023;
mod problem_024;
mod problem_025;
mod problem_277;

fn main() {
	use std::time::Instant;
	let now;
	let elapsed;
	let answer: u64;
	let answers: HashMap<u16, u64> = HashMap::from([
		(1, 233168), (2, 4613732), (3, 6857), (4, 906609), (5, 232792560), (6, 25164150), (7, 104743), (8, 23514624000), (9, 31875000), (10, 142913828922),
		(11, 70600674), (12, 76576500), (13, 5537376230), (14, 837799), (15, 137846528820), (16, 1366), (17, 21124), (18, 1074), (19, 171), (20, 648),
		(21, 31626), (22, 871198282), (23, 4179871), (24, 2783915460), (25, 4782), (67, 7273), (277, 1125977393124310)]);

	let mut user_input = String::new();
	print!("\nProblem No. to run: ");
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut user_input).unwrap();
	let mut sanitized: String = user_input.chars().filter(|c| c.is_digit(10)).collect();
	if sanitized.is_empty() {sanitized = "0".to_string()}
	let problem_no: u16 = sanitized.parse::<u16>().unwrap();
	now = Instant::now();

	match problem_no {
		1 => {			
			answer = problem_001::multiples_of_3_or_5(1_000);
			elapsed = now.elapsed();
			println!("(1) Multiples of 3 or 5: {:.2?} ", elapsed);
		}

		2 => {			
			answer = problem_002::even_fibonacci_numbers(4_000_000);
			elapsed = now.elapsed();
			println!("(2) Even Fibonacci numbers: {:.2?}", elapsed);
		}

		3 => {			
			answer = problem_003::largest_prime_factor(600_851_475_143);
			elapsed = now.elapsed();
			println!("(3) Largest prime factor: {:.2?}", elapsed);
		}

		4 => {			
			answer = problem_004::largest_palindrome_product(3);
			elapsed = now.elapsed();
			println!("(4) Largest palindrome product: {:.2?}", elapsed);
		}

		5 => {			
			answer = problem_005::smallest_multiple(20);
			elapsed = now.elapsed();
			println!("(5) Smallest multiple: {:.2?}", elapsed);
		}

		6 => {			
			answer = problem_006::sum_square_difference(100) as u64;
			elapsed = now.elapsed();
			println!("(6) Sum square difference: {:.2?}", elapsed);
		}

		7 => {			
			answer = problem_007::nth_prime(10001) as u64;
			elapsed = now.elapsed();
			println!("(7) 10001st prime: {:.2?}", elapsed);
		}

		8 => {			
			answer = problem_008::largest_product_in_a_series();
			elapsed = now.elapsed();
			println!("(8) Largest product in a series: {:.2?}", elapsed);
		}

		9 => {			
			answer = problem_009::special_pythagorean_triplet() as u64;
			elapsed = now.elapsed();
			println!("(9) Special Pythagorean triplet: {:.2?}", elapsed);
		}

		10 => {			
			answer = problem_010::summation_of_primes();
			elapsed = now.elapsed();
			println!("(10) Summation of primes: {:.2?}", elapsed);
		}

		11 => {			
			answer = problem_011::largest_product_in_a_grid() as u64;
			elapsed = now.elapsed();
			println!("(11) Largest product in a grid: {:.2?}", elapsed);
		}

		12 => {			
			answer = problem_012::highly_divisible_triangular_number() as u64;
			elapsed = now.elapsed();
			println!("(12) Highly divisible triangular number: {:.2?}", elapsed);
		}

		13 => {			
			answer = problem_013::large_sum();
			elapsed = now.elapsed();
			println!("(13) Large sum: {:.2?}", elapsed);
		}

		14 => {			
			answer = problem_014::longest_collatz_sequence() as u64;
			elapsed = now.elapsed();
			println!("(14) Longest Collatz sequence: {:.2?}", elapsed);
		}

		15 => {			
			answer = problem_015::lattice_paths(20u8);
			elapsed = now.elapsed();
			println!("(15) Lattice paths: {:.2?}", elapsed);
		}

		16 => {			
			answer = problem_016::power_digit_sum();
			elapsed = now.elapsed();
			println!("(16) Power digit sum: {:.2?}", elapsed);
		}

		17 => {			
			answer = problem_017::number_letter_counts() as u64;
			elapsed = now.elapsed();
			println!("(17) Number letter counts: {:.2?}", elapsed);
		}

		18 => {
			unsafe {
				answer = problem_018::maximum_path_sum_i("src/problem_018.txt") as u64;
				elapsed = now.elapsed();
				println!("(18) Maximum path sum I: {:.2?}", elapsed);
			}
		}

		19 => {			
			answer = problem_019::counting_sundays() as u64;
			elapsed = now.elapsed();
			println!("(19) Counting Sundays: {:.2?}", elapsed);
		}

		20 => {			
			answer = problem_020::factorial_digit_sum(100) as u64;
			elapsed = now.elapsed();
			println!("(20) Factorial digit sum: {:.2?}", elapsed);
		}

		21 => {			
			answer = problem_021::amicable_numbers() as u64;
			elapsed = now.elapsed();
			println!("(21) Amicable numbers: {:.2?}", elapsed);
		}

		22 => {			
			answer = problem_022::names_scores() as u64;
			elapsed = now.elapsed();
			println!("(22) Names scores: {:.2?}", elapsed);
		}

		23 => {
			answer = problem_023::non_abundant_sums() as u64;
			elapsed = now.elapsed();
			println!("(23) Non-abundant sums: {:.2?}", elapsed);
		}

		24 => {			
			answer = problem_024::lexicographic_permutations(1000000) as u64;
			elapsed = now.elapsed();
			println!("(24) Lexicographic permutations: {:.2?}", elapsed);
		}

		25 => {			
			answer = problem_025::one_thousand_digit_fibonacci_number() as u64;
			elapsed = now.elapsed();
			println!("(25) Lexicographic permutations: {:.2?}", elapsed);
		}

		67 => {
			unsafe {	
				answer = problem_018::maximum_path_sum_i("src/problem_067.txt") as u64;
				elapsed = now.elapsed();
				println!("(67) Maximum path sum II: {:.2?}", elapsed);
			}
		}

		277 => {			
			answer = problem_277::a_modified_collatz_sequence();
			elapsed = now.elapsed();
			println!("(277) A Modified Collatz sequence: {:.2?}", elapsed);
		}
		
		_ => {answer = 0; println!("Problem not found")}
	}

	if answer != *answers.get(&problem_no).unwrap() {
		println!("Incorrect Solution!\n");
	}
}