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

    let mut sum = 0;

    for i in left_list {
        let aa = right_list.iter().filter(|x| **x == i).count();
        sum += aa as i32 * i;
    }

    println!("{}", sum)
}

// 21070419
