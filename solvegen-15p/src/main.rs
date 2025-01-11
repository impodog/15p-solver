use solvegen_15p::*;

fn compute_database() {
    let _ = DB_LIST.is_empty();
}

fn main() {
    let handle = std::thread::spawn(compute_database);

    println!("Input your 15-puzzle layout, use 0 for the space:");

    let mut puzzle = Vec::new();
    'input: loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("stdin error");
        for number in line.split_whitespace() {
            match number.parse::<Value>() {
                Ok(number) => {
                    if number < SQUARE_LENGTH {
                        puzzle.push(number);
                        if puzzle.len() == SQUARE_LENGTH_USIZE {
                            break 'input;
                        }
                    } else {
                        println!("{} is too large", number);
                    }
                }
                Err(err) => {
                    println!("{:?} unaccepted: {}", number, err);
                }
            }
        }
    }
    let mut array = [Value::default(); SQUARE_LENGTH_USIZE];
    for (index, value) in puzzle.into_iter().enumerate() {
        array[index] = value;
    }

    let puzzle = Puzzle::from_slider(array);

    handle.join().unwrap();

    println!("Estimated solution length: {}", puzzle.heu());
    let result = Node::new(puzzle).rbfs();
    match result {
        Some(solution) => {
            let mut list = Vec::new();
            for slide in solution.0.into_iter() {
                let ch = match slide {
                    Slide::Left => 'L',
                    Slide::Right => 'R',
                    Slide::Up => 'U',
                    Slide::Down => 'D',
                };
                list.push(ch);
            }
            if let Some(ch) = list.first() {
                println!(
                    "Solution found with {} {}:",
                    list.len(),
                    if list.len() == 1 { "step" } else { "steps" }
                );
                print!("{ch}");
                for ch in list.iter().skip(1) {
                    print!(" {ch}");
                }
            } else {
                print!("The puzzle is already solved");
            }
            println!();
        }
        _ => {
            println!("No solution found!");
        }
    }
}
