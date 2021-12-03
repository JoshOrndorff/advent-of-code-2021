use std::fs;

fn main() {
    let text = fs::read_to_string("../input/day1.txt")
        .expect("Input file should be read successfully");

    let depths = text
        .lines()
        .map(|s| u32::from_str_radix(s, 10).expect("input should parse as a number correctly"))
        .collect::<Vec<_>>();

    let mut window_depths = Vec::<u32>::new();
    for i in 1..depths.len() - 1 {
        let window_depth = depths[i - 1] + depths[i] + depths[i + 1];
        window_depths.push(window_depth);
    }
    
    // Solve Part 1
    let mut deepenings = 0;
    let mut previous_depth = None;
    for depth in depths {
        if let Some(prev) = previous_depth {
            if prev < depth {
                deepenings += 1;
            }
        }

        previous_depth = Some(depth);
    }

    println!("The ground gets deeper {:?} times", deepenings);

    // Solve Part 2
    let mut deepenings = 0;
    let mut previous_depth = None;
    for depth in window_depths {
        if let Some(prev) = previous_depth {
            if prev < depth {
                deepenings += 1;
            }
        }

        previous_depth = Some(depth);
    }

    println!("The ground gets deeper {:?} times", deepenings);
}
