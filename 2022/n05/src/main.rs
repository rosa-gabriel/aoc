// Separate the file in the 2 parts and decide how to store the parts:
// * Descer até a linha dos números e começar o processo de separação;

// Identificar pilha em cada linha:
// *  

// Organizar cada pilha de containers em stacks:


// Processo de troca
// Pop de 1 -> Push em outro

// 

// Loop through the move count and execute the command
fn main() {
    let input_content = std::fs::read_to_string("input.txt").unwrap();

    let result: Vec<&str> = input_content
        .split("\n\n")
        .collect::<Vec<&str>>();


    let mut upper_text: Vec<&str> = result
        .get(0)
        .unwrap()
        .split("\n")
        .collect();

    let commands_text: Vec<&str> = result
        .get(1)
        .unwrap()
        .split("\n")
        .filter(|x| {
            x.len() > 0
        })
        .collect();

    let size: usize = upper_text
        .pop()
        .unwrap()
        .replace(" ", "")
        .pop()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..size {
        stacks.push(Vec::new());
    }

    upper_text.reverse();
    for line in upper_text.iter() {
        let bytes_on_line = line.as_bytes();

        let mut word_index: i32 = 1;
        let mut current_word: usize = 0;

        for c in bytes_on_line {
            match word_index {
                2 => {
                    if *c as char  != ' ' {
                        stacks[current_word].push(*c as char);
                    }
                    word_index += 1;
                },
                3 => {
                    word_index = 0;
                    current_word += 1;
                },
                _ => word_index += 1
            }
        }
    }

    let command_number: Vec<Vec<i32>> = commands_text
        .iter()
        .map(|x| {
            let replaced_x = x
                .replace("move ", "")
                .replace(" from ", "-")
                .replace(" to ", "-");

            return replaced_x
                .split("-")
                .map(|y| {
                    return y.parse().unwrap();
                })
                .collect();
        })
        .collect();

    for command in command_number.iter() {
        let ammount = command[0];
        let start = command[1];
        let target = command[2];

        let mut stack: Vec<char> = vec![];

        for _ in 0..ammount {
            stack.push(stacks[start as usize -1].pop().unwrap());
        }

        stack.reverse();

        for i in stack {
            stacks[target as usize -1].push(i);
        }

        // Part 1 Res
        // for _ in 0..ammount {
        //     let value = stacks[start as usize -1].pop().unwrap();
        //     stacks[target as usize -1].push(value);
        // }
    }

    for i in stacks.iter_mut() {
        println!("{}", i.pop().unwrap());
    }

    // println!("{:#?}", stacks);
}
