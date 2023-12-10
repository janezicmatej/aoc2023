use std::collections::HashSet;

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn next_move(dy: isize, dx: isize, c: char) -> Option<(isize, isize)> {
    Some(match (dy, dx) {
        (1, 0) => match c {
            '|' => (1, 0),
            'J' => (0, -1),
            'L' => (0, 1),
            _ => None?,
        },
        (0, 1) => match c {
            '-' => (0, 1),
            '7' => (1, 0),
            'J' => (-1, 0),
            _ => None?,
        },
        (-1, 0) => match c {
            '|' => (-1, 0),
            '7' => (0, -1),
            'F' => (0, 1),
            _ => None?,
        },
        (0, -1) => match c {
            '-' => (0, -1),
            'F' => (1, 0),
            'L' => (-1, 0),
            _ => None?,
        },
        _ => unreachable!(),
    })
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let pipes: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let start = pipes
        .iter()
        .enumerate()
        .find_map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(|(j, y)| (i, j, y))
                .find(|(_, _, &x)| x == 'S')
        })
        .map(|(j, i, _)| (j as isize, i as isize))
        .unwrap();

    (pipes, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pipes, start) = parse_input(input);

    'start_dir: for d in DIRS {
        let (mut dy, mut dx) = d;
        let (mut ly, mut lx) = (start.0 + d.0, start.1 + d.1);
        let mut counter = 0;

        while start != (ly, lx) {
            let p = match pipes.get(ly as usize).and_then(|x| x.get(lx as usize)) {
                Some(x) => *x,
                None => continue 'start_dir,
            };

            match next_move(dy, dx, p) {
                Some((ndy, ndx)) => (dy, dx) = (ndy, ndx),
                None => continue 'start_dir,
            }

            counter += 1;
            (ly, lx) = (ly + dy, lx + dx);
        }

        return Some((counter + 1) / 2);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pipes, start) = parse_input(input);

    let mut border = HashSet::with_capacity(pipes.len() * pipes[0].len());

    'start_dir: for d in DIRS {
        border.clear();
        border.insert(start);

        let (mut dy, mut dx) = d;
        let (mut ly, mut lx) = (start.0 + d.0, start.1 + d.1);

        while start != (ly, lx) {
            let p = match pipes.get(ly as usize).and_then(|x| x.get(lx as usize)) {
                Some(x) => *x,
                None => continue 'start_dir,
            };

            match next_move(dy, dx, p) {
                Some((ndx, ndy)) => {
                    (dy, dx) = (ndx, ndy);
                    border.insert((ly, lx));
                }
                None => continue 'start_dir,
            }

            (ly, lx) = (ly + dy, lx + dx);
        }

        break;
    }

    let mut counter = 0;

    for (sy, sx) in (1..pipes.len())
        .map(|x| (x, 0))
        .chain((0..pipes[0].len()).map(|x| (0, x)))
    {
        let mut inside = false;
        for range in 0.. {
            if let Some(c) = pipes.get(sy + range).and_then(|x| x.get(sx + range)) {
                let is_border = border.contains(&((sy + range) as isize, (sx + range) as isize));
                if is_border && !['7', 'L'].contains(c) {
                    inside = !inside;
                }
                if !is_border && inside {
                    counter += 1;
                }
            } else {
                break;
            }
        }
    }

    Some(counter)
}

aoc::solution!(10);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file_part("examples", 10, 1)),
            Some(8)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file_part("examples", 10, 2)),
            Some(10)
        );
    }
}
