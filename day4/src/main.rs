#[derive(Debug)]
/// A bingo board that can be marked and checked for wins
struct Board {
    /// A 5X5 2D table where each cell has a number and is either marked or not.
    data: [[(u32, bool); 5]; 5],
}

impl Board {
    /// Takes in a multiline string slice.
    /// Rows are newline delimited, Columns are space delimited.
    fn new(s: &str) -> Self {
        let mut data = [[(0u32, false); 5]; 5];

        let lines = s.lines().collect::<Vec<_>>();
        for i in 0usize..5 {
            let entries = lines[i]
                .split_ascii_whitespace()
                .map(|x| (x.parse::<u32>().unwrap(), false))
                .collect::<Vec<_>>();
            data[i] = entries[0..5].try_into().unwrap();
        }

        Self { data }
    }

    /// Try to mark a square on the board
    fn mark(&mut self, target: u32) {
        //todo maybe return something.
        // could be true if something. was marked
        // could do an automatic win check and return the puzzle answer
        for (n, m) in self.data.iter_mut().flatten() {
            if target == *n {
                *m = true;
                // Breaking is an optimization that assumes a given number appears at most once on a single board.
                // If this is not true (or I end up in doubt) I can just remove the break to be sure.
                break;
            }
        }
    }

    /// Check if the board is currently winning
    fn is_winning(&self) -> bool {
        // Check rows first
        for row in self.data {
            let mut winning = true;
            for col in row {
                if !col.1 {
                    winning = false;
                    break;
                }
            }
            if winning {
                return true;
            }
        }

        // Check columns next
        for col_index in 0usize..5 {
            let mut winning = true;
            for row in self.data {
                if !row[col_index].1 {
                    winning = false;
                    break;
                }
            }
            if winning {
                return true;
            }
        }

        // If no wins found in rows or columns, that's it
        false
    }

    /// Returns the sum of all unmarked tiles.
    /// Useful in calculating the final answer to the problem
    fn sum_of_unmarked(&self) -> u32 {
        self.data
            .iter()
            .flatten()
            .filter_map(|(n, marked)| if !marked { Some(n) } else { None })
            .sum()
    }
}

fn main() {
    let raw_input =
        std::fs::read_to_string("../input/day4.txt").expect("input file should exist");

    let mut input_iter = raw_input.split("\n\n");
    let draws = input_iter
        .next()
        .expect("first line is drawn numbers, so it should exist")
        .split(',')
        .map(|n| n.parse::<u32>().unwrap());

    let mut boards = input_iter.map(Board::new).collect::<Vec<_>>();

    'outer: for draw in draws {
        for b in &mut boards {
            b.mark(draw);
            if b.is_winning() {
                let score = b.sum_of_unmarked() * draw;
                println!("Winning board with a score of {}!", score);
                break 'outer;
            }
        }
    }
}
