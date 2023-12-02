pub fn part_one(input: &str) -> Option<u32> {
    let mut c = 0;

    for line in input.lines() {
        let mut itr = line.chars().filter(char::is_ascii_digit);

        let first = itr.next().unwrap();
        let last = itr.last().unwrap_or(first);

        c += format!("{first}{last}").parse::<u32>().unwrap();
    }

    Some(c)
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    // NOTE:(matej) this solution is O(n^2) since we search string for each substring separately

    let mut c = 0;
    let nums: Vec<(u32, String)> = (1..=9).zip(NUMS.into_iter().map(String::from)).collect();

    for line in input.lines() {
        let mut all_matches = Vec::new();

        for (n, str_n) in nums.iter() {
            all_matches.extend(line.match_indices(str_n).map(|(x, _)| (x, *n)));
        }

        all_matches.extend(
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit())
                .map(|(idx, c)| (idx, c.to_digit(10).unwrap())),
        );

        let first = all_matches.iter().min_by_key(|&(idx, _)| idx).unwrap().1;
        let last = all_matches.iter().max_by_key(|&(idx, _)| idx).unwrap().1;

        c += first * 10 + last;
    }

    Some(c)
}

aoc::solution!(1);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 1)),
            Some(351)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 1)),
            Some(340)
        );
    }
}
