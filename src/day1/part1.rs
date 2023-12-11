pub fn run() -> u32{
    let lines = super::utils::INPUT.lines();

    let mut sum = 0;

    for line in lines {
        let result = get_calibration_val(line);
        if result != None {
            sum += result.unwrap();
        }
    }

    return sum;
}

fn get_calibration_val(text: &str) -> Option<u32> {
    let mut first_digit = None;
    let mut last_digit = None;

    for ch in text.chars() {
        if ch.is_digit(10) {
            last_digit = ch.to_digit(10);
            if first_digit.is_none() {
                first_digit = last_digit;
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some(first * 10 + last),
        _ => None,
    }
}