use std::collections::VecDeque;

fn find_history(v: VecDeque<i32>) -> VecDeque<VecDeque<i32>> {
    let mut s = VecDeque::new();
    s.push_back(v);

    loop {
        let mut all_zeros = true;

        let last = s.back().unwrap();
        let mut new = VecDeque::new();
        for i in 0..(last.len() - 1) {
            let diff = last[i + 1] - last[i];
            if diff != 0 {
                all_zeros = false;
            }

            new.push_back(diff);
        }
        s.push_back(new);

        if all_zeros {
            break;
        }
    }

    s
}

fn extrapolate_forward(v: VecDeque<i32>) -> i32 {
    let mut s = find_history(v);

    for i in (1..s.len()).rev() {
        let adder = *s[i].back().unwrap();
        let last = *s[i - 1].back().unwrap();
        s[i - 1].push_back(last + adder);
    }

    *s[0].back().unwrap()
}

fn extrapolate_back(v: VecDeque<i32>) -> i32 {
    let mut s = find_history(v);

    for i in (1..s.len()).rev() {
        let adder = *s[i].front().unwrap();
        let first = *s[i - 1].front().unwrap();
        s[i - 1].push_front(first - adder);
    }

    *s[0].front().unwrap()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|x| x.split(' ').filter_map(|y| y.parse().ok()).collect())
            .map(extrapolate_forward)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|x| x.split(' ').filter_map(|y| y.parse().ok()).collect())
            .map(extrapolate_back)
            .sum(),
    )
}

aoc::solution!(9);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 9)), None);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 9)), None);
    }
}
