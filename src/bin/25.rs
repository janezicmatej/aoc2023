use std::collections::HashMap;

fn build_graph(input: &str, skips: &[(&str, &str)]) -> Vec<Vec<usize>> {
    let mut nodes = Vec::new();
    let mut mapper = HashMap::new();

    for (node, neighs) in input.lines().filter_map(|x| x.split_once(": ")) {
        if !mapper.contains_key(node) {
            mapper.insert(node, nodes.len());
            nodes.push(Vec::new());
        }

        let index_a = mapper[node];

        for n in neighs.split(' ') {
            if !mapper.contains_key(n) {
                mapper.insert(n, nodes.len());
                nodes.push(Vec::new());
            }

            if skips.contains(&(node, n)) || skips.contains(&(n, node)) {
                continue;
            }

            let index_b = mapper[n];
            nodes[index_a].push(index_b);
            nodes[index_b].push(index_a);
        }
    }

    nodes
}

pub fn part_one_wrapped(input: &str, skips: &[(&str, &str)]) -> Option<usize> {
    let nodes = build_graph(input, skips);

    let mut visited = vec![false; nodes.len()];
    let mut stack = vec![0];

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        for n in nodes[node].iter().copied() {
            if !visited[n] {
                stack.push(n);
            }
        }
    }

    let counter = visited.iter().copied().filter(|&x| x).count();
    Some((nodes.len() - counter) * counter)
}

pub fn part_one(input: &str) -> Option<usize> {
    // to figure out which nodes to skip
    // echo "graph A {\n$(cat data/inputs/25.txt)\n}" | \
    // sed 's/\(.*\): \(.*\)$/\1 -- {\2}/' | \
    // dot -Tsvg -Kneato > graph.svg
    let skips = [("sfm", "vmt"), ("vph", "mfc"), ("fql", "rmg")];
    part_one_wrapped(input, &skips)
}

pub fn part_two(_input: &str) -> Option<String> {
    Some("Happy chrismas!".into())
}

aoc::solution!(25);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let skips = [("pzl", "hfx"), ("bvb", "cmg"), ("nvd", "jqt")];
        let input = aoc::template::read_file("examples", 25);
        assert_eq!(part_one_wrapped(&input, &skips), Some(54));
    }
}
