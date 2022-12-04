fn main() {
    let input = std::fs::read_to_string("../input/day7example.txt").expect("input file exists");

    let positions = input.trim().split(',').map(|s| s.parse::<i32>().unwrap());

    let max_starting = positions
        .clone()
        .max()
        .expect("There is at least one value, so we can calculate a max");

    // Part 1
    let lowest_linear_cost = (0..max_starting)
        .map(|i| {
            positions
                .clone()
                .map(|start| i32::abs(start - i))
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("lowest linear cost: {}", lowest_linear_cost);

    // Part 2
    let lowest_quadratic_cost = (0..max_starting)
        .map(|i| {
            positions
                .clone()
                .map(|start| i32::abs(start - i))
                .map(|distance| distance * (distance + 1) / 2)
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("lowest quadratic cost: {}", lowest_quadratic_cost);
}
