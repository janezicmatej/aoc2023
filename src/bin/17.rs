use std::{
    collections::{BinaryHeap, HashMap},
    ops::Neg,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    heat: usize,
    position: (usize, usize),
    direction: Direction,
    steps: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn find_path(
    grid: &[Vec<usize>],
    start: (usize, usize),
    target: (usize, usize),
    min_steps: usize,
    max_steps: usize,
) -> Option<usize> {
    let valid_indexing = |y: usize, x: usize| grid.get(y).and_then(|row| row.get(x)).is_some();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        heat: 0,
        position: (0, 0),
        direction: Direction::Right,
        steps: 0,
    });

    let mut heatmap = HashMap::new();
    heatmap.insert((start, Direction::Down, 0), 0);
    heatmap.insert((start, Direction::Up, 0), 0);

    while let Some(State {
        heat,
        position,
        direction,
        steps,
    }) = heap.pop()
    {
        if position == target {
            return Some(heat);
        }

        if *heatmap
            .get(&(position, direction, steps))
            .unwrap_or(&usize::MAX)
            < heat
        {
            continue;
        }

        let (y, x) = position;
        for d in Direction::ALL.iter().filter(|&x| *x != -direction) {
            if steps < min_steps && *d != direction {
                continue;
            }

            let (ny, nx) = match d {
                Direction::Up => (y.wrapping_sub(1), x),
                Direction::Down => (y + 1, x),
                Direction::Left => (y, x.wrapping_sub(1)),
                Direction::Right => (y, x + 1),
            };

            if !valid_indexing(ny, nx) {
                continue;
            }

            let new_steps = if *d == direction { steps + 1 } else { 1 };

            if new_steps > max_steps {
                continue;
            }

            let state = State {
                heat: heat + grid[ny][nx],
                position: (ny, nx),
                direction: *d,
                steps: new_steps,
            };

            if *heatmap
                .get(&((ny, nx), *d, new_steps))
                .unwrap_or(&usize::MAX)
                > state.heat
            {
                heatmap.insert(((ny, nx), *d, new_steps), state.heat);
                heap.push(state);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|x| {
            x.chars()
                .filter_map(|x| x.to_digit(10).map(|x| x as usize))
                .collect()
        })
        .collect();

    let target = (grid.len() - 1, grid[0].len() - 1);

    find_path(&grid, (0, 0), target, 0, 3)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|x| {
            x.chars()
                .filter_map(|x| x.to_digit(10).map(|x| x as usize))
                .collect()
        })
        .collect();

    let target = (grid.len() - 1, grid[0].len() - 1);

    find_path(&grid, (0, 0), target, 4, 10)
}

aoc::solution!(17);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 17)),
            Some(102)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 17)),
            Some(94)
        );
    }
}
