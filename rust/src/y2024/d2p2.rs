use std::fs;

pub fn run() -> () {
    let input_content = fs::read_to_string("input.txt").unwrap();

    let result: i32 = input_content
        .lines()
        .map(|i| {
            let mut prev_level_opt: Option<i32> = None;
            let mut cresc: Option<bool> = None;

            for level in i.split(" ") {
                let curr_num = level.parse().unwrap();

                if prev_level_opt.is_none() {
                    prev_level_opt = Some(curr_num);
                    continue;
                }

                let prev_level = prev_level_opt.unwrap();

                if cresc.is_none() {
                    cresc = Some(curr_num > prev_level);
                }
                let cresc = cresc.unwrap();

                let diff: i32 = (prev_level as i32 - curr_num as i32).abs();

                if !(((cresc && prev_level < curr_num) || (!cresc && prev_level > curr_num))
                    && (diff <= 3 && diff >= 1))
                {
                    return 0;
                }

                prev_level_opt = Some(curr_num);
            }

            return 1;
        })
        .sum();

    println!("{:?}", result);
}

// 
