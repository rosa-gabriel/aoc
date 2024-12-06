use std::fs;

pub fn run() -> () {
    let input_content = fs::read_to_string("input.txt").unwrap();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    input_content.split("\n").for_each(|i| {
        if i == "" {
            return;
        }

        let items: Vec<&str> = i.split("   ").collect();
        let i1: i32 = items[0].parse().unwrap();
        let i2: i32 = items[1].parse().unwrap();

        left_list.push(i1);
        right_list.push(i2);
    });

    left_list.sort();
    right_list.sort();

    let mut sum: i32 = 0;

    for i in 0..left_list.len() {
        let i1 = left_list[i];
        let i2 = right_list[i];

        sum += (i1 - i2).abs();
    }

    println!("{}", sum)
}

// 1223326
