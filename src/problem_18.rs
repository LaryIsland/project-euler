use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::Read;

static mut PYRAMID: Vec<Vec<u16>> = Vec::new();

pub unsafe fn maximum_path_sum_i(file_path: &str) -> u16 {
	PYRAMID.clear();
	let mut data: String = String::new();
    match File::open(file_path) {
        Ok(mut file) => {
            file.read_to_string(&mut data).unwrap();
        },
        Err(error) => {
            println!("Error opening file {}: {}", file_path, error);
        },
    }

	for line in data.lines() {
		PYRAMID.push(line.split(" ").map(|x| x.parse().expect("Not an integer!")).collect());
	}
		
	let mut memoize: HashMap<(u8, u8), u16> = HashMap::new();

	unsafe fn recursive_search(row: u8, index: u8, memoize: &mut HashMap<(u8, u8), u16>) -> u16 {
		if row == 0 {
			return PYRAMID[usize::from(row)][usize::from(index)]
		}
		if memoize.contains_key(&(row, index)) {
			return *memoize.get(&(row, index)).unwrap();
		}
		if index + 1 < PYRAMID[usize::from(row)].len() as u8 {
			let right = recursive_search(row - 1, index, memoize);
			memoize.insert((row - 1, index), right);
		}
		if index != 0 {
			let left = recursive_search(row - 1, index - 1, memoize);
			memoize.insert((row - 1, index - 1), left);
		}
		if index == 0 {
			return PYRAMID[usize::from(row)][usize::from(index)] + recursive_search(row - 1, index, memoize);
		}
		if index + 1 == PYRAMID[usize::from(row)].len() as u8 {
			return PYRAMID[usize::from(row)][usize::from(index)] + recursive_search(row - 1, index - 1, memoize);
		}
		return PYRAMID[usize::from(row)][usize::from(index)] + cmp::max(recursive_search(row - 1, index, memoize), recursive_search(row - 1, index - 1, memoize));
	}

	let mut max = 0;
	for i in 0..PYRAMID.len() as u8 {
		let tmp = recursive_search((PYRAMID.len() - 1) as u8, i, &mut memoize);
		if tmp > max {
			max = tmp;
		}
	}
	return max;
}