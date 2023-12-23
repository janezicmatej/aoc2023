use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Default)]
struct Node {
    neighbours: Vec<(usize, usize)>,
}

fn build_graph(input: &str, two_way: bool) -> (Vec<Node>, usize) {
    let mut grid: Vec<Vec<_>> = input.lines().map(|x| x.as_bytes().to_vec()).collect();

    let l = grid[0].len();

    let border: Vec<_> = once(b'#').cycle().take(l).collect();
    grid.insert(0, border.clone());
    grid.push(border);

    let start = (1, 1);
    let end = (grid.len() - 2, grid[0].len() - 2);

    let mut nodes = Vec::new();

    let mut mapper = HashMap::new();
    mapper.insert(start, 0);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut stack = Vec::new();
    // (from_node, at, len)
    stack.push((start, start, 0));

    let forced = |x| match x {
        b'>' => (0, 1),
        b'v' => (1, 0),
        _ => (0, 0),
    };

    while let Some((from, (y, x), len)) = stack.pop() {
        let is_node = DIRS
            .iter()
            .filter(|(dy, dx)| {
                grid[((y as isize) + dy) as usize][((x as isize) + dx) as usize] != b'#'
            })
            .count()
            > 2
            || (y, x) == start
            || (y, x) == end;

        if is_node {
            if !visited.contains(&(y, x)) {
                mapper.insert((y, x), nodes.len());
                nodes.push(Node::default());
            }

            if from != (y, x) {
                nodes[mapper[&from]].neighbours.push((mapper[&(y, x)], len));
                if two_way {
                    nodes[mapper[&(y, x)]].neighbours.push((mapper[&from], len));
                }
            }
        }

        if !visited.insert((y, x)) {
            continue;
        }
        for (ny, nx, _, _) in DIRS
            .iter()
            .map(|(dy, dx)| {
                (
                    ((y as isize) + dy) as usize,
                    ((x as isize) + dx) as usize,
                    dy,
                    dx,
                )
            })
            .filter(|(ny, nx, _, _)| grid[*ny][*nx] != b'#')
            .filter(|(ny, nx, dy, dx)| {
                let (fy, fx) = forced(grid[*ny][*nx]);
                (fy + **dy, fx + **dx) != (0, 0)
            })
        {
            let new_len = if is_node { 1 } else { len + 1 };
            let new_from = if is_node { (y, x) } else { from };
            stack.push((new_from, (ny, nx), new_len));
        }
    }

    (nodes, mapper[&end])
}

fn longest_path(
    nodes: &[Node],
    mut visited: usize,
    location: usize,
    target: usize,
    length: usize,
) -> Option<usize> {
    if location == target {
        return Some(length);
    }

    // binary mask for visited since < 64 nodes in input
    // nth bit tells if location n was visited already
    visited |= 1 << location;

    let mut max_len = 0;

    for (n, l) in nodes[location]
        .neighbours
        .iter()
        .copied()
        .filter(|(n, _)| (visited & 1 << n) == 0)
    {
        if let Some(new_len) = longest_path(nodes, visited, n, target, length + l) {
            if max_len < new_len {
                max_len = new_len;
            }
        }
    }

    if max_len > 0 {
        Some(max_len)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (nodes, target) = build_graph(input, false);
    longest_path(&nodes, 0, 0, target, 0)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (nodes, target) = build_graph(input, true);
    longest_path(&nodes, 0, 0, target, 0)
}

aoc::solution!(23);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 23)),
            Some(94)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 23)),
            Some(154)
        );
    }
}
