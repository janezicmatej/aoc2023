use std::collections::HashSet;

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// type is (output dy, output dx, (left neighbour, expand), (right neighbour, expand))
// we need to "expand" one point into three when neighbour tile is on a curve eg.: right side of
// seven going up
type MoveMapper = (isize, isize, ((isize, isize), bool), ((isize, isize), bool));

fn next_move(dy: isize, dx: isize, c: char) -> Option<MoveMapper> {
    Some(match (dy, dx) {
        (1, 0) => match c {
            '|' => (1, 0, ((0, 1), false), ((0, -1), false)),
            'J' => (0, -1, ((1, 1), true), ((-1, -1), false)),
            'L' => (0, 1, ((-1, 1), false), ((1, -1), true)),
            _ => None?,
        },
        (0, 1) => match c {
            '-' => (0, 1, ((-1, 0), false), ((1, 0), false)),
            '7' => (1, 0, ((-1, 1), true), ((1, -1), false)),
            'J' => (-1, 0, ((-1, -1), false), ((1, 1), true)),
            _ => None?,
        },
        (-1, 0) => match c {
            '|' => (-1, 0, ((0, -1), false), ((0, 1), false)),
            '7' => (0, -1, ((1, -1), false), ((-1, 1), true)),
            'F' => (0, 1, ((-1, -1), true), ((1, 1), false)),
            _ => None?,
        },
        (0, -1) => match c {
            '-' => (0, -1, ((1, 0), false), ((-1, 0), false)),
            'F' => (1, 0, ((1, 1), false), ((-1, -1), true)),
            'L' => (-1, 0, ((1, -1), true), ((-1, 1), false)),
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

    let max_y = pipes.len() as isize;
    let max_x = pipes[0].len() as isize;
    let invalid_indexing = |ny, nx| ny < 0 || nx < 0 || ny >= max_y || nx >= max_x;

    'start_dir: for d in DIRS {
        let (mut dy, mut dx) = d;
        let (mut ly, mut lx) = (start.0 + d.0, start.1 + d.1);
        let mut counter = 0;

        while start != (ly, lx) {
            if invalid_indexing(ly, lx) {
                continue 'start_dir;
            }

            let p = pipes[ly as usize][lx as usize];

            match next_move(dy, dx, p) {
                Some((ndy, ndx, _, _)) => (dy, dx) = (ndy, ndx),
                None => continue 'start_dir,
            }

            counter += 1;
            (ly, lx) = (ly + dy, lx + dx);
        }

        return Some((counter + 1) / 2);
    }

    None
}

fn dfs(
    starts: &HashSet<(isize, isize)>,
    border: &HashSet<(isize, isize)>,
    (max_y, max_x): (isize, isize),
) -> Option<u32> {
    let mut queue = Vec::from_iter(starts.iter().copied());
    let mut visited = HashSet::new();

    let invalid_indexing = |ny, nx| ny < 0 || nx < 0 || ny >= max_y || nx >= max_x;

    while let Some((ny, nx)) = queue.pop() {
        if invalid_indexing(ny, nx) {
            return None;
        }
        if visited.contains(&(ny, nx)) {
            continue;
        }
        if border.contains(&(ny, nx)) {
            continue;
        }

        visited.insert((ny, nx));

        for (dy, dx) in DIRS {
            queue.push((ny + dy, nx + dx));
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pipes, start) = parse_input(input);

    let max_y = pipes.len() as isize;
    let max_x = pipes[0].len() as isize;
    let invalid_indexing = |ny, nx| ny < 0 || nx < 0 || ny >= max_y || nx >= max_x;

    let mut lrb = [HashSet::new(), HashSet::new(), HashSet::new()];

    'start_dir: for d in DIRS {
        for map in lrb.iter_mut() {
            map.clear();
        }

        lrb[2].insert(start);

        let (mut dy, mut dx) = d;
        let (mut ly, mut lx) = (start.0 + d.0, start.1 + d.1);

        while start != (ly, lx) {
            if invalid_indexing(ly, lx) {
                continue 'start_dir;
            }

            let p = pipes[ly as usize][lx as usize];
            match next_move(dy, dx, p) {
                Some((ndx, ndy, ((lly, llx), lexpand), ((rry, rrx), rexpand))) => {
                    (dy, dx) = (ndx, ndy);
                    lrb[2].insert((ly, lx));

                    lrb[0].insert((ly + lly, lx + llx));
                    if lexpand {
                        lrb[0].insert((ly + lly, lx));
                        lrb[0].insert((ly, lx + llx));
                    }

                    lrb[1].insert((ly + rry, lx + rrx));
                    if rexpand {
                        lrb[1].insert((ly + rry, lx));
                        lrb[1].insert((ly, lx + rrx));
                    }
                }
                None => continue 'start_dir,
            }

            (ly, lx) = (ly + dy, lx + dx);
        }

        break;
    }

    dfs(&lrb[0], &lrb[2], (max_y, max_x)).or(dfs(&lrb[1], &lrb[2], (max_y, max_x)))
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
