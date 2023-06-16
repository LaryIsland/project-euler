pub fn names_scores() -> u32 {
	let mut sorted_names = Vec::new();

	let mut data: String = String::new();
    match std::fs::File::open("src/problem_022.txt") {
        Ok(mut file) => {
            std::io::Read::read_to_string(&mut file, &mut data).unwrap();
        },
        Err(error) => {
            println!("Error opening file {}: {}", "problem_022.txt", error);
        },
    }

	for name in data.split(',') {
		sorted_names.push(name.trim_matches('"'));
	}

	sorted_names.sort();

	let mut total_sum = 0u32;
	let mut counter = 0u32;
	let mut name_sum;
	for name in sorted_names {
		name_sum = 0u32;
		counter += 1;
		for letter in name.chars() {
			name_sum += letter as u32 - 64; //-64 converts uppercase UTF-8 to their integer value i.e. A -> 1, B -> 2 ... Z -> 26
		}
		total_sum += name_sum * counter;
	}
	return total_sum;
}