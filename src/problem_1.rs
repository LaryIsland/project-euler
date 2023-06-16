pub fn multiples_of_3_or_5(limit: u64) -> u64 {
    let maximum = (limit - 1) as f64;
    let mut total = 0;

    total += series_sum((maximum/3.0).floor()) * 3;
    total += series_sum((maximum/5.0).floor()) * 5;
    total -= series_sum((maximum/15.0).floor()) * 15;

    return total;
}

fn series_sum(series: f64) -> u64 {
    return (series/2.0 * (1.0 + series)) as u64;
}