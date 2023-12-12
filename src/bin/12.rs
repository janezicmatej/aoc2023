use std::{collections::HashMap, iter::once};

fn is_valid(sequence: &[char], ptr: usize, group: usize) -> bool {
    let edges_front = *once(&'.').chain(sequence.iter()).nth(ptr).unwrap() != '#';
    let edges_back = *sequence.iter().chain(once(&'.')).nth(ptr + group).unwrap() != '#';

    let filled = sequence
        .iter()
        .chain(once(&'.').cycle())
        .skip(ptr)
        .take(group)
        .all(|x| *x != '.');

    edges_front && edges_back && filled
}

fn count(
    memo: &mut HashMap<(usize, usize), usize>,
    sequence: &[char],
    groups: &[usize],
    ptr: usize,
) -> usize {
    match groups.split_first() {
        None => !sequence.iter().skip(ptr).any(|c| *c == '#') as usize,
        Some((group, r_groups)) => {
            let remaining = r_groups.iter().sum();
            let mut total = 0;

            for idx in ptr..(sequence.len() - group - remaining + 1) {
                if is_valid(sequence, idx, *group) {
                    let next = idx + *group + 1;
                    match memo.get(&(remaining, next)) {
                        Some(m) => total += m,
                        None => {
                            let count = count(memo, sequence, r_groups, next);
                            memo.insert((remaining, next), count);
                            total += count
                        }
                    }
                }

                if sequence[idx] == '#' {
                    break;
                }
            }
            total
        }
    }
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (str_seq, str_grp) = line.split_once(' ').unwrap();
    let sequence = str_seq.chars().collect();
    let groups = str_grp.split(',').filter_map(|x| x.parse().ok()).collect();

    (sequence, groups)
}

fn unfold(sequence: Vec<char>, groups: Vec<usize>, n: usize) -> (Vec<char>, Vec<usize>) {
    let seq_len = sequence.len();
    let grp_len = groups.len();

    let new_sequence = sequence
        .into_iter()
        .chain(once('?'))
        .cycle()
        .take(seq_len * n + n - 1)
        .collect();
    let new_groups = groups.into_iter().cycle().take(grp_len * n).collect();

    (new_sequence, new_groups)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(sequence, groups)| count(&mut HashMap::new(), &sequence, &groups, 0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(parse_line)
            .map(|(sequence, groups)| unfold(sequence, groups, 5))
            .map(|(sequence, groups)| count(&mut HashMap::new(), &sequence, &groups, 0))
            .sum(),
    )
}

aoc::solution!(12);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 12)),
            Some(21)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 12)),
            Some(525152)
        );
    }
}
