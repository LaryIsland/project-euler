pub fn number_letter_counts() -> u32 {
		let one = 3;
		let two = 3;
		let three = 5;
		let four = 4;
		let five = 4;
		let six = 3;
		let seven = 5;
		let eight = 5;
		let nine = 4;
		let ten = 3;
		let eleven = 6;
		let twelve = 6;
		let thirteen = 8;
		let fourteen = 8;
		let fifteen = 7;
		let sixteen = 7;
		let seventeen = 9;
		let eighteen = 8;
		let nineteen = 8;
		let twenty = 6;
		let thirty = 6;
		let forty = 5;
		let fifty = 5;
		let sixty = 5;
		let seventy = 7;
		let eighty = 6;
		let ninety = 6;
		let hundred = 7;
		let thousand = 8;
		let and = 3;

	let mut sum = 0;
	let mut digits: Vec<char> = vec![];

	for i in 1..1001 {
		digits.clear();

		if i < 10 {
			digits.push('0');
			digits.push('0');
		} else if i < 100 {
			digits.push('0');
		}
		
		for digit in i.to_string().chars() {
			digits.push(digit);
		}
		let mut special = false;
		
		if digits.len() == 3 {
			match digits[0] {
				'1' => sum += one + hundred + and,
				'2' => sum += two + hundred + and,
				'3' => sum += three + hundred + and,
				'4' => sum += four + hundred + and,
				'5' => sum += five + hundred + and,
				'6' => sum += six + hundred + and,
				'7' => sum += seven + hundred + and,
				'8' => sum += eight + hundred + and,
				'9' => sum += nine + hundred + and,
				_ => {}
			}
			
			match digits[1] {
				'1' => {
					special = true;
					match digits[2] {
						'0' => sum += ten,
						'1' => sum += eleven,
						'2' => sum += twelve,
						'3' => sum += thirteen,
						'4' => sum += fourteen,
						'5' => sum += fifteen,
						'6' => sum += sixteen,
						'7' => sum += seventeen,
						'8' => sum += eighteen,
						'9' => sum += nineteen,
						_ => {}
					}
				}
				'2' => sum += twenty,
				'3' => sum += thirty,
				'4' => sum += forty,
				'5' => sum += fifty,
				'6' => sum += sixty,
				'7' => sum += seventy,
				'8' => sum += eighty,
				'9' => sum += ninety,
				_ => {}
			}

			if !special {
				match digits[2] {
					'1' => sum += one,
					'2' => sum += two,
					'3' => sum += three,
					'4' => sum += four,
					'5' => sum += five,
					'6' => sum += six,
					'7' => sum += seven,
					'8' => sum += eight,
					'9' => sum += nine,
					'0' => if digits[1] == '0' {
						sum -= and
					},
					_ => {}
				}
			}
		}
		else {
			sum += one + thousand
		}
	}
    return sum;
}