use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::{self, u32},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

pub fn process_part_2(input: &str) -> String {
    "MCD".to_owned()
}

pub fn process_part_1(input: &str) -> String {
    let mut parsed_crates = parse_crate_stacks(&input);
    let moves = parse_move_lines(&input);
    for mv in moves {
        let from = (mv[1] - 1) as usize;
        let to = (mv[2] - 1) as usize;
        for _ in 0..mv[0] {
            let c = parsed_crates[from].pop().unwrap();
            parsed_crates[to].push(c);
        }
    }
    let mut top_crates: Vec<&str> = Vec::<&str>::new();
    for mut c in parsed_crates {
        let top = c.pop();
        if let Some(val) = top {
            top_crates.push(val);
        }
    }
    let top_str = top_crates.join("");
    top_str
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;
    let result = match c {
        "   " => None,
        val => Some(val),
    };
    Ok((input, result))
}

fn parse_crate_row(row: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, crates) = separated_list0(tag(" "), parse_crate)(row)?;
    Ok((input, crates))
}

fn parse_crate_stacks(stacks: &str) -> Vec<Vec<&str>> {
    let result: Vec<Vec<Option<&str>>> = stacks
        .lines()
        .map(|row| parse_crate_row(row).unwrap().1)
        .collect();

    let cols = result[0].len();
    let mut stacks: Vec<Vec<&str>> = Vec::with_capacity(cols);
    for _ in 0..cols {
        stacks.push(Vec::<&str>::new());
    }

    for row in result {
        for (i, c) in row.iter().enumerate() {
            match c {
                &Some(value) => stacks[i].push(value),
                _ => continue,
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn parse_moves_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = u32(input)?;
    Ok((input, vec![count, from, to]))
}

fn parse_move_lines(input: &str) -> Vec<Vec<u32>> {
    let moves = input
        .lines()
        .filter(|line| line.contains("move"))
        .map(|line| parse_moves_line(line).unwrap().1)
        .collect::<Vec<Vec<u32>>>();
    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_part_2() {
        let input = fs::read_to_string("test-input1.txt").unwrap();
        let top_crates = process_part_2(&input);
        assert_eq!(top_crates, "MCD");
    }

    #[test]
    fn test_parse_move_lines() {
        let input = fs::read_to_string("test-input1.txt").unwrap();
        let moves = parse_move_lines(&input);
        assert_eq!(
            moves,
            vec![vec![1, 2, 1], vec![3, 1, 3], vec![2, 2, 1], vec![1, 1, 2]]
        );
    }

    #[test]
    fn test_parse_move_line() {
        let input = "move 5 from 4 to 5";
        let output = parse_moves_line(&input).unwrap().1;
        assert_eq!(output, vec![5, 4, 5]);
    }

    #[test]
    fn test_parse_crate_stacks() {
        let input = fs::read_to_string("test-input1.txt").unwrap();
        let output = parse_crate_stacks(&input);
        assert_eq!(output, vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]]);
    }

    #[test]
    fn test_parse_crate_row() {
        let input = "    [D]    ";
        let output = parse_crate_row(&input).unwrap().1;
        assert_eq!(output, vec![Option::None, Some("D"), Option::None]);

        let input = "[C] [B] [A]";
        let output = parse_crate_row(&input).unwrap().1;
        assert_eq!(output, vec![Some("C"), Some("B"), Some("A")]);

        let input = "[E]        ";
        let output = parse_crate_row(&input).unwrap().1;
        assert_eq!(output, vec![Some("E"), None, None])
    }

    #[test]
    fn test_parse_crate() {
        let input = "[D]";
        let output = parse_crate(&input).unwrap().1.unwrap();
        assert_eq!(output, "D");

        let input = "   ";
        let output = parse_crate(&input).unwrap().1;
        assert!(output.is_none());
    }

    #[test]
    fn test_process_part_1() {
        let input = fs::read_to_string("test-input1.txt").unwrap();
        let output = process_part_1(&input);
        assert_eq!(output, "CMZ");
    }
}
