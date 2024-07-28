pub fn run() -> () {
    let file_contents = std::fs::read_to_string("input.txt").unwrap();
    let result: i32 = file_contents
        .split("\n")
        .map(|item| {
            let item_length = item.len();
            if item_length == 0 {
                return 0;
            }

            let first_sub = &item[0..item_length/2];
            let second_sub = &item[item_length/2..item_length];
            return find_char_value(find_common_character(first_sub, second_sub));
        })
        .sum();
    println!("{:#?}", result);
}

fn find_common_character(arr1: &str, arr2: &str) -> u8 {
    for item_in_arr_1 in arr1.as_bytes() {
        for item_in_arr2 in arr2.as_bytes()  {
            if item_in_arr_1 == item_in_arr2 { return *item_in_arr_1 };
        }
    }

    return 0;
}

fn find_char_value(letter: u8) -> i32 {
    let subtrahend: i32;

    if (letter as char).is_lowercase() {
        subtrahend = 96;
    } else {
        subtrahend = 38;
    }

    return (letter as i32) - subtrahend;
}
