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
	use num_bigint::BigUint;
	let run_previous = true;
	let mut now;
	let mut elapsed;

if run_previous {
    now = Instant::now();
	let problem_1 = problem_001::multiples_of_3_or_5(1_000);
	elapsed = now.elapsed();
    println!("(1) Multiples of 3 or 5: {:.2?} ", elapsed);
	if problem_1 != 233168 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_2 = problem_002::even_fibonacci_numbers(4_000_000);
	elapsed = now.elapsed();
    println!("(2) Even Fibonacci numbers: {:.2?}", elapsed);
	if problem_2 != 4613732 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_3 = problem_003::largest_prime_factor(600_851_475_143);
	elapsed = now.elapsed();
    println!("(3) Largest prime factor: {:.2?}", elapsed);
	if problem_3 != 6857 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_4 = problem_004::largest_palindrome_product(3);
	elapsed = now.elapsed();
    println!("(4) Largest palindrome product: {:.2?}", elapsed);
	if problem_4 != 906609 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_5 = problem_005::smallest_multiple(20);
	elapsed = now.elapsed();
    println!("(5) Smallest multiple: {:.2?}", elapsed);
	if problem_5 != 232792560 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_6 = problem_006::sum_square_difference(100);
	elapsed = now.elapsed();
    println!("(6) Sum square difference: {:.2?}", elapsed);
	if problem_6 != 25164150 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_7 = problem_007::nth_prime(10001);
	elapsed = now.elapsed();
    println!("(7) 10001st prime: {:.2?}", elapsed);
	if problem_7 != 104743 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_8 = problem_008::largest_product_in_a_series();
	elapsed = now.elapsed();
    println!("(8) Largest product in a series: {:.2?}", elapsed);
	if problem_8 != 23514624000 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_9 = problem_009::special_pythagorean_triplet();
	elapsed = now.elapsed();
    println!("(9) Special Pythagorean triplet: {:.2?}", elapsed);
	if problem_9 != 31875000 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_10 = problem_010::summation_of_primes();
	elapsed = now.elapsed();
    println!("(10) Summation of primes: {:.2?}", elapsed);
	if problem_10 != 142913828922 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_11 = problem_011::largest_product_in_a_grid();
	elapsed = now.elapsed();
    println!("(11) Largest product in a grid: {:.2?}", elapsed);
	if problem_11 != 70600674 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_12 = problem_012::highly_divisible_triangular_number();
	elapsed = now.elapsed();
    println!("(12) Highly divisible triangular number: {:.2?}", elapsed);
	if problem_12 != 76576500 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_13 = problem_013::large_sum();
	elapsed = now.elapsed();
    println!("(13) Large sum: {:.2?}", elapsed);
	if problem_13 != 5537376230 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_14 = problem_014::longest_collatz_sequence();
	elapsed = now.elapsed();
    println!("(14) Longest Collatz sequence: {:.2?}", elapsed);
	if problem_14 != 837799 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_15 = problem_015::lattice_paths(20u8);
	elapsed = now.elapsed();
    println!("(15) Lattice paths: {:.2?}", elapsed);
	if problem_15 != BigUint::from(137846528820u64) {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_16 = problem_016::power_digit_sum();
	elapsed = now.elapsed();
    println!("(16) Power digit sum: {:.2?}", elapsed);
	if problem_16 != BigUint::from(1366u16) {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_17 = problem_017::number_letter_counts();
	elapsed = now.elapsed();
    println!("(17) Number letter counts: {:.2?}", elapsed);
	if problem_17 != 21124 {
		println!("Incorrect Solution!\n");
	}
	
	unsafe {
		now = Instant::now();
		let problem_18 = problem_018::maximum_path_sum_i("src/problem_018.txt");
		elapsed = now.elapsed();
		println!("(18) Maximum path sum I: {:.2?}", elapsed);
		if problem_18 != 1074 {
			println!("Incorrect Solution!\n");
		}
	}

	now = Instant::now();
	let problem_19 = problem_019::counting_sundays();
	elapsed = now.elapsed();
    println!("(19) Counting Sundays: {:.2?}", elapsed);
	if problem_19 != 171 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_20 = problem_020::factorial_digit_sum(100);
	elapsed = now.elapsed();
    println!("(20) Factorial digit sum: {:.2?}", elapsed);
	if problem_20 != 648 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_21 = problem_021::amicable_numbers();
	elapsed = now.elapsed();
    println!("(21) Amicable numbers: {:.2?}", elapsed);
	if problem_21 != 31626 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_22 = problem_022::names_scores();
	elapsed = now.elapsed();
    println!("(22) Names scores: {:.2?}", elapsed);
	if problem_22 != 871198282 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_23 = problem_023::non_abundant_sums();
	elapsed = now.elapsed();
    println!("(23) Non-abundant sums: {:.2?}", elapsed);
	if problem_23 != 4179871 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_24 = problem_024::lexicographic_permutations(1000000);
	elapsed = now.elapsed();
    println!("(24) Lexicographic permutations: {:.2?}", elapsed);
	if problem_24 != 2783915460 {
		println!("Incorrect Solution!\n");
	}

	now = Instant::now();
	let problem_25 = problem_025::one_thousand_digit_fibonacci_number();
	elapsed = now.elapsed();
    println!("(25) Lexicographic permutations: {:.2?}", elapsed);
	if problem_25 != 4782 {
		println!("Incorrect Solution!\n");
	}

	unsafe {
		now = Instant::now();
		let problem_67 = problem_018::maximum_path_sum_i("src/problem_067.txt");
		elapsed = now.elapsed();
		println!("(67) Maximum path sum II: {:.2?}", elapsed);
		if problem_67 != 7273 {
			println!("Incorrect Solution!\n{0}", problem_67);
		}
	}
}

	// now = Instant::now();
	// let problem_277 = problem_277::a_modified_collatz_sequence();
	// elapsed = now.elapsed();
	// println!("(277) A Modified Collatz sequence: {:.2?}", elapsed);
	// println!("{0}", problem_277);
	// if problem_277 != 0 {
	// 	println!("Incorrect Solution!\n");
	// }
}