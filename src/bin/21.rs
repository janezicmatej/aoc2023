use std::{collections::HashSet, usize};

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (isize, isize)) {
    let mut grid: Vec<_> = input.lines().map(|x| x.as_bytes().to_vec()).collect();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(y, item)| item.iter().enumerate().map(move |(x, item)| (x, y, *item)))
        .find_map(|(x, y, s)| if s == b'S' { Some((y, x)) } else { None })
        .unwrap();

    grid[start.0][start.1] = b'.';

    (grid, (start.0 as isize, start.1 as isize))
}

fn walk_return_at(
    grid: &[Vec<u8>],
    start: (isize, isize),
    mut returns: Vec<usize>,
    can_cycle: bool,
) -> Vec<usize> {
    returns.sort_by(|a, b| b.cmp(a));

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let invalid_indexing = |y, x| y < 0 || y >= h || x < 0 || x >= w;

    let mut results = Vec::new();
    let length = returns[0];
    let mut next = returns.pop().unwrap();

    let mut visited = HashSet::new();
    visited.insert(start);

    for i in 1..=length {
        let mut new_visited = HashSet::new();

        for (y, x) in visited.iter() {
            for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (ny, nx) = (y + dy, x + dx);

                if !can_cycle && invalid_indexing(ny, nx) {
                    continue;
                }

                let (cy, cx) = (ny.rem_euclid(h) as usize, nx.rem_euclid(w) as usize);

                if grid[cy][cx] == b'.' {
                    new_visited.insert((ny, nx));
                }
            }
        }

        visited = new_visited;

        if i == next {
            results.push(visited.len());
            if !returns.is_empty() {
                next = returns.pop().unwrap();
            }
        }
    }

    results
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start) = parse_input(input);
    let result = walk_return_at(&grid, start, vec![64], false);

    Some(result[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, start) = parse_input(input);

    let h = grid.len();
    let s = start.0 as usize;

    let result = walk_return_at(&grid, start, vec![s, s + h, s + 2 * h], true);

    let a = (result[2] - 2 * result[1] + result[0]) / 2;
    let b = (result[1] - result[0]) - a;
    let c = result[0];

    let x = 26501365 / h;

    Some(a * x * x + b * x + c)
}

aoc::solution!(21);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 21)),
            Some(42)
        );
    }
}
