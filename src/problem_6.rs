pub fn sum_square_difference(limit: u32) -> u32 {
	let sum_of_squares = ( limit * ( limit + 1 ) * ( 2 * limit + 1 ) ) / 6;
	let square_of_sum = ( limit / 2 * ( 1 + limit ) ).pow(2);
	return square_of_sum - sum_of_squares;
}