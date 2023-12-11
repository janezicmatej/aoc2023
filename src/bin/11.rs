use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<(usize, usize)>, HashSet<usize>, HashSet<usize>) {
    let mut galaxies = Vec::new();
    let mut rows = HashSet::new();
    let mut columns = HashSet::from_iter(1..(input.lines().next().unwrap().len()));

    for (y, line) in input.lines().enumerate() {
        let mut found = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((y, x));
                found = true;
            }
            columns.remove(&x);
        }
        if !found {
            rows.insert(y);
        }
    }

    (galaxies, rows, columns)
}

fn solve(input: &str, explode: usize) -> Option<usize> {
    let (galaxies, rows, columns) = parse_input(input);

    let mut counter = 0;

    for ((y1, x1), (y2, x2)) in galaxies
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, x)| galaxies.iter().skip(i + 1).copied().map(move |y| (x, y)))
    {
        let y_abs = y1.abs_diff(y2);
        let x_abs = x1.abs_diff(x2);
        let x_extra = columns
            .iter()
            .filter(|x| (x1..=x2).contains(x) || (x2..=x1).contains(x))
            .count();
        let y_extra = rows
            .iter()
            .filter(|y| (y1..=y2).contains(y) || (y2..=y1).contains(y))
            .count();
        counter += x_abs + y_abs + (x_extra + y_extra) * (explode - 1);
    }

    Some(counter)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1_000_000)
}

aoc::solution!(11);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 11)),
            Some(374)
        );
    }
    #[test]
    fn test_part_two() {
        let input = aoc::template::read_file("examples", 11);
        assert_eq!(solve(&input, 10), Some(1030));
        assert_eq!(solve(&input, 100), Some(8410));
    }
}
