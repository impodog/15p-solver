use solvegen_15p::*;

fn compute_database() {
    // Start the computation of database
    let _ = DB_LIST.is_empty();
}

enum Method {
    Astar,
    Rbfs,
}

fn solve() {
    println!("Choose a search method:\n1. A*\n2. RBFS");
    let method = {
        let mut method = String::new();
        std::io::stdin().read_line(&mut method).unwrap();
        match method.trim() {
            "1" => Method::Astar,
            "2" => Method::Rbfs,
            _ => {
                println!("Unknown method: {method:?}, using A*");
                Method::Astar
            }
        }
    };

    println!("Input the weight factor(>=1.0), bigger for less optimized solution, or just press Enter to get optimal result:");
    let weight = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("stdin error");
        if line.chars().all(|ch| ch.is_whitespace()) {
            None
        } else {
            line.trim()
                .parse::<f32>()
                .inspect_err(|err| {
                    println!("{}, using default", err);
                })
                .ok()
        }
    };
    {
        let mut config = CONFIG.write().unwrap();
        *config = Config { weight };
    }

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

    let result = match method {
        Method::Astar => astar::AstarNode::new(puzzle, Default::default()).astar(),
        Method::Rbfs => rbfs::RbfsNode::new(puzzle, Default::default()).rbfs(),
    };

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

fn generate() {
    println!("Input approximate desired solve steps:");
    let mut steps = String::new();
    std::io::stdin().read_line(&mut steps).unwrap();
    let steps = match steps.trim().parse::<usize>() {
        Ok(steps) => steps,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let puzzle = Puzzle::new_random_approx(steps);
    let mut count = 0;
    println!("Result:");
    for value in puzzle.pos_to_slider() {
        print!("{value}");

        count += 1;
        if count == 4 {
            count = 0;
            println!();
        } else {
            print!(" ");
        }
    }
}

fn main() {
    let _handle = std::thread::spawn(compute_database);

    loop {
        println!("--------------------\n1. Solve\n2. Generate");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => solve(),
            "2" => generate(),
            _ => {
                println!("Choice {choice:?} not available");
            }
        }
    }
}
