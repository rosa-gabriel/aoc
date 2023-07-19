use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let result = file.split("\n").collect::<Vec<&str>>();

    let mut highest: i32 = 0;
    let mut current: i32 = 0;

    for item in result.iter() {
        if item == &"" {
            highest = if current > highest { current } else { highest };
            current = 0;
        } else {
            let number_value: i32 = item.parse().unwrap();

            current += number_value;
        }
    };

    println!("{:?}", highest);
}
