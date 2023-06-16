pub fn counting_sundays() -> u32 {
	let years = 100;
	let days = years * 365;
	let leap_days = years / 4 - 1;
	return ((days + leap_days) / 7) * 2 / 61;
}