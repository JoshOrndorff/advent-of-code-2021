// 312158273346506965865627867367495394155is too high

use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("../input/day6.txt").expect("input file exists");

    // Make a new double ended queue with nine buckets (indexed 0 - 8)
    let mut buckets = VecDeque::<u64>::from([0u64; 9]);

    input.trim().split(',').map(|s| s.parse().unwrap()).for_each(|n| buckets[n] += 1);

    for n in 1..=256 {
        // Lantern fish with 0 days remaining are spawning now
        let spawning_now = buckets.pop_front().expect("There should always be 9 elements in the queue");

        // The parents are put back into the queue with 6 days left before their next spawn
        buckets[6] += spawning_now;

        // The children are added to the very end with 8 days remaining
        buckets.push_back(spawning_now);

        println!("after {} days the population is {}", n, buckets.iter().sum::<u64>());
    }
    
}
