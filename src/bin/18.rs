use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        Ok(match s {
            "U" | "3" => Up,
            "D" | "1" => Down,
            "L" | "2" => Left,
            "R" | "0" => Right,
            _ => return Err(ParseDirectionError),
        })
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        use Direction::*;
        match value {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

fn get_area(border: &[(isize, isize)], border_length: isize) -> isize {
    // get area with shoelace formula (trapezoid variant)
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut shoelace: isize = 0;
    for n in 0..border.len() {
        let (y1, x1) = border[n];
        let (y2, x2) = border[(n + 1) % border.len()];
        shoelace += (y1 + y2) * (x1 - x2);
    }
    let area = shoelace / 2;

    // get interior by inverting pick's theorem formula
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let interior = area + 1 - border_length / 2;
    interior + border_length
}

fn get_border(instructions: &[(Direction, isize)]) -> (Vec<(isize, isize)>, isize) {
    let mut border = Vec::new();
    let mut border_length = 0;
    let (mut sy, mut sx) = (0, 0);
    border.push((sy, sx));

    for (d, l) in instructions.iter().copied() {
        let (dy, dx) = d.into();
        (sy, sx) = (sy + l * dy, sx + l * dx);
        border.push((sy, sx));
        border_length += l;
    }

    border.pop();

    (border, border_length)
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions: Vec<(Direction, isize)> = input
        .lines()
        .map(|line| {
            let [d, l, _] = line.splitn(3, ' ').collect::<Vec<_>>().try_into().unwrap();
            (d.parse().unwrap(), l.parse::<isize>().unwrap())
        })
        .collect();

    let (border, border_length) = get_border(&instructions);
    Some(get_area(&border, border_length))
}

fn join_option_tuple<T, U>((a, b): (Option<T>, Option<U>)) -> Option<(T, U)> {
    Some((a?, b?))
}

pub fn part_two(input: &str) -> Option<isize> {
    let instructions: Vec<(Direction, isize)> = input
        .lines()
        .filter_map(|line| line.split_once(" (#"))
        .filter_map(|(_, h)| h.strip_suffix(')'))
        .map(|h| h.split_at(h.len() - 1))
        .map(|(hex, dir)| (dir.parse().ok(), isize::from_str_radix(hex, 16).ok()))
        .filter_map(join_option_tuple)
        .collect();

    let (border, border_length) = get_border(&instructions);
    Some(get_area(&border, border_length))
}

aoc::solution!(18);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 18)),
            Some(62)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 18)),
            Some(952408144115)
        );
    }
}
