use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, (y, x): (usize, usize)) -> (usize, usize) {
        use Direction::*;
        match self {
            Up => (y.checked_sub(1).unwrap_or(usize::MAX), x),
            Down => (y + 1, x),
            Left => (y, x.checked_sub(1).unwrap_or(usize::MAX)),
            Right => (y, x + 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mirror {
    Vertical,
    Horizontal,
    EvenSymmetric,
    OddSymmetric,
}

struct ParseMirrorError;

impl TryFrom<u8> for Mirror {
    type Error = ParseMirrorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'\\' => Self::EvenSymmetric,
            b'/' => Self::OddSymmetric,
            _ => return Err(ParseMirrorError),
        })
    }
}

impl Mirror {
    fn bounce(&self, d: &Direction) -> (Direction, Option<Direction>) {
        use Direction::*;
        use Mirror::*;

        match self {
            Vertical => match d {
                Up | Down => (*d, None),
                Left | Right => (Up, Some(Down)),
            },
            Horizontal => match d {
                Up | Down => (Left, Some(Right)),
                Left | Right => (*d, None),
            },
            EvenSymmetric => (
                match d {
                    Up => Left,
                    Down => Right,
                    Left => Up,
                    Right => Down,
                },
                None,
            ),
            OddSymmetric => (
                match d {
                    Up => Right,
                    Down => Left,
                    Left => Down,
                    Right => Up,
                },
                None,
            ),
        }
    }
}

fn solve_with_start(layout: &[Vec<Option<Mirror>>], start: (usize, usize, Direction)) -> usize {
    let mut queue = vec![start];
    let mut visited = HashSet::new();

    while let Some((y, x, d)) = queue.pop() {
        let point = layout.get(y).and_then(|row| row.get(x));

        if point.is_none() {
            continue;
        }

        if !visited.insert((y, x, d)) {
            continue;
        }

        if let Some(Some(m)) = point {
            let (new_d, opt_new_d) = m.bounce(&d);
            let (ny, nx) = new_d.next((y, x));
            queue.push((ny, nx, new_d));

            if let Some(od) = opt_new_d {
                let (ony, onx) = od.next((y, x));
                queue.push((ony, onx, od));
            }
        } else {
            let (ny, nx) = d.next((y, x));
            queue.push((ny, nx, d));
        }
    }

    HashSet::<(usize, usize)>::from_iter(visited.into_iter().map(|(y, x, _)| (y, x))).len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let layout: Vec<Vec<_>> = input
        .lines()
        .map(|x| x.as_bytes().iter().map(|&x| x.try_into().ok()).collect())
        .collect();

    Some(solve_with_start(&layout, (0, 0, Direction::Right)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let layout: Vec<Vec<_>> = input
        .lines()
        .map(|x| x.as_bytes().iter().map(|&x| x.try_into().ok()).collect())
        .collect();

    let h = layout.len();
    let w = layout[0].len();

    let mut scores = Vec::with_capacity(2 * (h + w) + 1);

    for hh in 0..h {
        let left = solve_with_start(&layout, (hh, 0, Direction::Right));
        let right = solve_with_start(&layout, (hh, w - 1, Direction::Left));

        scores.push(left);
        scores.push(right);
    }

    for ww in 0..w {
        let downward = solve_with_start(&layout, (0, ww, Direction::Down));
        let upward = solve_with_start(&layout, (h - 1, ww, Direction::Up));

        scores.push(downward);
        scores.push(upward);
    }

    scores.into_iter().max()
}

aoc::solution!(16);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 16)),
            Some(46)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 16)),
            Some(51)
        );
    }
}
