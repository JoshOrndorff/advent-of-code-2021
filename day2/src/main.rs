use std::fs;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    sequence::{preceded, pair},
    character::complete::{digit1, space1},
    multi::many1,
};

#[derive(Debug)]
pub enum Move {
    Up(u32),
    Down(u32),
    Forward(u32),
}

pub fn parse_move(input: &str) 
-> IResult<&str, Move> {
    alt((up, down, forward))(input)
}

fn up(input: &str) -> IResult<&str, Move> {
    let (rest, value) = preceded(
        pair(tag("up"), space1),
        many1(digit1)
    )(input)?; 
    Ok((
        rest,
        Move::Up(
            u32::from_str_radix(
              &value.join(""),
              10,
          )
          .expect("How to handle the case where the number is too big...")
      )
      ))
}

fn down(input: &str) -> IResult<&str, Move> {
    // TODO consider map_res rather than let
    let (rest, value) = preceded(
        pair(tag("down"), space1),
        many1(digit1)
    )(input)?; 
    Ok((
      rest,
      Move::Down(
          u32::from_str_radix(
            &value.join(""),
            10,
        )
        .expect("How to handle the case where the number is too big...")
    )
    ))
}

// TODO combine these into a single function that takes the tag as a param
fn forward(input: &str) -> IResult<&str, Move> {
    let (rest, value) = preceded(
        pair(tag("forward"), space1),
        many1(digit1)
    )(input)?; 
    Ok((
        rest,
        Move::Forward(
            u32::from_str_radix(
              &value.join(""),
              10,
          )
          .expect("How to handle the case where the number is too big...")
      )
      ))
}

#[derive(Default, Debug)]
struct Position {
    depth: u32,
    forward: u32,
    aim: u32,
}

impl Position {
    fn part1_move(&self, m: &Move) -> Self {
        match m {
            Move::Up(by) => Self { depth: self.depth - by, forward: self.forward, aim: self.aim },
            Move::Down(by) => Self { depth: self.depth + by, forward: self.forward, aim: self.aim },
            Move::Forward(by) => Self { depth: self.depth, forward: self.forward + by, aim: self.aim },
        }
    }

    fn part2_move(&self, m: &Move) -> Self {
        match m {
            Move::Up(by) => Self { depth: self.depth, forward: self.forward, aim: self.aim  - by},
            Move::Down(by) => Self { depth: self.depth, forward: self.forward, aim: self.aim + by },
            Move::Forward(by) => Self { depth: self.depth + by * self.aim, forward: self.forward + by, aim: self.aim },
        }
    }
}

fn main() {
    let text = fs::read_to_string("../input/day2.txt")
    .expect("Input file should be read successfully");

    let moves = text
        .lines()
        .map(|s| parse_move(s).expect("input should be valid").1)
        .collect::<Vec<_>>();

    let mut part1_position = Position::default();
    let mut part2_position = Position::default();
    for m in &moves {
        part1_position = part1_position.part1_move(m);
        part2_position = part2_position.part2_move(m);
    }

    println!("Part 1 result is {:?}", part1_position.depth * part1_position.forward);
    println!("Part 2 result is {:?}", part2_position.depth * part2_position.forward);
}
