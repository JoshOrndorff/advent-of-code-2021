use std::collections::HashMap;

type Coords = ((u32, u32), (u32, u32));

/// We model the Sea Floor as a 2D grid. The grid is stored as a hashmap
/// from coordinates to the number of lines that cover that point
#[derive(Default)]
struct SeaFloor {
    data: HashMap<(u32, u32), u32>,
}

impl SeaFloor {
    /// Get the number of lines that cover a given point. Basically just
    /// looks up in the hashmap, but also handles some default value and unwrapping stuff
    fn lines_at_point(&self, coords: &(u32, u32)) -> u32 {
        *self.data.get(coords).unwrap_or(&0)
    }

    /// Create a new SeaFloor map from the given lines
    fn add_line(&mut self, line: Coords, diags: bool) {
        match line {
            ((x, y1), (x2, y2)) if x == x2 => {
                // Vertical line
                let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
                for y in range {
                    self.data.entry((x, y)).and_modify(|n| *n += 1).or_insert(1);
                }
            }
            ((x1, y), (x2, y2)) if y == y2 => {
                // Horizontal line
                let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
                for x in range {
                    self.data.entry((x, y)).and_modify(|n| *n += 1).or_insert(1);
                }
            }
            ((x1, y1), (x2, y2)) => {
                // Line is diagonal (45 degrees guaranteed by problem)
                if !diags {
                    // In part 1 we don't consider diagonal lines, so return without doing anything
                    return;
                }
                // Start at the left-most end
                let (x_start, y_start, uphill, length) = if x1 < x2 {
                    (x1, y1, y2 > y1, x2 - x1)
                } else {
                    (x2, y2, y1 > y2, x1 - x2)
                };

                for offset in 0..=length {
                    let x = x_start + offset;
                    let y = if uphill { y_start + offset } else { y_start - offset };
                    self.data.entry((x, y)).and_modify(|n| *n += 1).or_insert(1);
                }
            }
        }
    }

    /// Prints out a visual representation of the sea floor like the one shown in the problem
    #[allow(dead_code)]
    fn visualize(&self) {
        // Find the max dimensions
        let (max_x, max_y) = self
            .data
            .keys()
            .fold((0u32, 0u32), |(x_max, y_max), (x, y)| {
                (
                    if x > &x_max { *x } else { x_max },
                    if y > &y_max { *y } else { y_max },
                )
            });

        for y in 0u32..=max_y {
            for x in 0u32..=max_x {
                let c = self.lines_at_point(&(x, y));
                print!("{}", c);
            }
            println!();
        }
    }
}

fn coords_from_string(s: &str) -> Coords {
    let first_comma = s.find(',').unwrap();
    let arrow = s.find(" -> ").unwrap();
    let second_comma = s.rfind(',').unwrap();

    let x1 = s[0..first_comma].parse().unwrap();
    let y1 = s[first_comma + 1..arrow].parse().unwrap();

    let x2 = s[arrow + 4..second_comma].parse().unwrap();
    let y2 = s[second_comma + 1..].parse().unwrap();

    ((x1, y1), (x2, y2))
}

fn main() {
    let input = std::fs::read_to_string("../input/day5example.txt").expect("input file should exist");

    // Create and populate a new sea floor
    let mut sea_floor = SeaFloor::default();
    input
        .lines()
        .map(coords_from_string)
        .for_each(|c| sea_floor.add_line(c, false));

    // Count how many cells have more than one line
    let part_1 = sea_floor
        .data
        .iter()
        .map(|(_, v)| v)
        .filter(|x| **x >= 2u32)
        .count();

    println!("part 1: {}", part_1);

    // ------------------

    // Create and populate a new sea floor
    let mut sea_floor = SeaFloor::default();
    input
        .lines()
        .map(coords_from_string)
        .for_each(|c| sea_floor.add_line(c, true));

    // Count how many cells have more than one line
    let part_1 = sea_floor
        .data
        .iter()
        .map(|(_, v)| v)
        .filter(|x| **x >= 2u32)
        .count();

    println!("part 1: {}", part_1);
}

#[test]
fn coords_1() {
    let result = coords_from_string(&"860,786 -> 701,945");

    assert_eq!(result, ((860, 786), (701, 945)));
}
