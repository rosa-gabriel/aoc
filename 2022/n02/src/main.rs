use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap(); 

    let result = file
        .split("\n")
        .map(|item| {
            let arr = item
                .split(" ")
                .collect::<Vec<&str>>();

            let a = arr[0];
            let b = arr[1];
        }).sum::<i32>();

    println!("{}", result);
}
