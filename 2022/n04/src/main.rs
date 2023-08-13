fn main() {
    let file_content = std::fs::read_to_string("input.txt").unwrap();

    let result: i32 = file_content
        .split("\n")
        .filter(|item| {
            item != &""
        })
        .map(|item| {
            let range_vector: Vec<Vec<i32>> = item
                .split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|number| {
                            number.parse::<i32>().unwrap()
                        })
                        .collect::<Vec<i32>>()
                })
                .collect();

            let first_range = &range_vector[0];
            let second_range = &range_vector[1];

            if first_range[0] == second_range[0] {
                return 1
            };

            if first_range[0] > second_range[0] {
                return if first_range[1] <= second_range[1] { 1 } else { 0 };
            } else if first_range[1] < second_range[1]{
                return 0;
            } else if first_range[1] >= second_range[1] {
                return 1;
            }

            return 0;
        })
        .sum();

    println!("{:#?}", result);
}
