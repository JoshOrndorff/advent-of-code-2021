use std::fs;

fn main() {
    let text = fs::read_to_string("../input/day3example.txt")
        .expect("Input file should be read successfully");

    let bit_strings = text
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    // Dynamically detect the dimensions so we can solve the example input and our real input
    let total_strings = bit_strings.len();
    let bit_length = bit_strings[0].len();
    println!("Total strings: {:?}", total_strings);
    println!("Bit length: {:?}", bit_length);

    // Count occurences
    let mut bitwise_counter: Vec<usize> = vec![0; bit_length];
    for bit_string in &bit_strings {
        for i in 0..bit_length {
            let c = bit_string[i];
            if c == '1' {
                bitwise_counter[i] += 1;
            }
        }
    }

    // Transform into target (most-common) bits
    // I'm ignoring the comment about if they are equal because there are
    // none that are equal in my data. I suspect that is the case in all
    // inputs because it would have also been relevant in part 1.
    let target_bits = bitwise_counter
        .iter()
        .map(|&count| if count > total_strings / 2 { '1' } else { '0' })
        .collect::<Vec<_>>();

    // If there are 1 bits in more than half the strings...
    let mut gamma_factor = vec_to_u32(&target_bits);
    let mut epsilon_factor = vec_to_u32(&bitwise_not(&target_bits));

    // Part 1 Results
    println!("gamma factor is {:?}", gamma_factor);
    println!("epsilon factor is {:?}", epsilon_factor);
    println!("product is {:?}", gamma_factor * epsilon_factor);

    // Look for the oxygen generator rating
    let mut oxygen_candidates = bit_strings.clone();
    for i in 0..bit_length {
        oxygen_candidates = oxygen_candidates.iter().cloned().filter(|s| s[i] == target_bits[i]).collect();
        if oxygen_candidates.len() == 1 {
            break;
        }
    }

    println!("oxygen bits {:?}", oxygen_candidates);
    println!("oxygen rating {:?}", vec_to_u32(&oxygen_candidates[0]));
}

fn vec_to_u32(input: &Vec<char>) -> u32 {
    let mut result = 0u32;
    let bit_length = input.len();
    for i in 0..bit_length {
        let exponent: u32 = bit_length as u32 - i as u32 - 1;
        if input[i] == '1' {
            result += 2u32.pow(exponent);
        }
    }
    result
}

fn bitwise_not(input: &Vec<char>) -> Vec<char> {
    let mut output = Vec::<char>::new();
    for c in input {
        output.push(
            match c {
                '1' => '0',
                '0' => '1',
                _ => panic!("I think I even saw a 2 in there!"),
            }
        )
    }
    output
}