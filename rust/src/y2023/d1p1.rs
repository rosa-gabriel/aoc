use std::fs;

pub fn run() -> () {
    let input_content = fs::read_to_string("../input.txt").unwrap();

    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    let mut total_sum: u32 = 0;

    for c in input_content.chars() {
        if c == '\n' {
            let Some(first_digit_value) = first_digit else { continue };
            let Some(last_digit_value) = last_digit else { continue };
            total_sum += std::str::from_utf8(&[first_digit_value as u8, last_digit_value as u8])
                .unwrap()
                .parse::<u32>()
                .unwrap();

            first_digit = None;
            last_digit = None;
        } else if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            } 

            last_digit = Some(c);
        };
    }

    println!("{}", total_sum);
}
