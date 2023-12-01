pub fn part_one(input: &str) -> Option<u32> {
    let mut c = 0;
    for line in input.lines() {
        let mut itr = line.chars().filter(|x| x.is_ascii_digit());

        let f = itr.next().unwrap();
        let l = itr.last().unwrap_or(f);

        c += format!("{f}{l}").parse::<u32>().unwrap();
    }

    Some(c)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut c = 0;
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut first = (line.len(), None);
        let mut last = (0, None);

        for (idx, n) in nums.iter().enumerate() {
            let matches: Vec<_> = line.match_indices(n).collect();

            if matches.is_empty() {
                continue;
            }

            let m = matches.first().unwrap();

            if m.0 < first.0 {
                first = (m.0, Some(idx + 1));
            }

            let m = matches.last().unwrap();

            if m.0 >= last.0 {
                last = (m.0, Some(idx + 1));
            }
        }
        let mut itr = line.chars().enumerate().filter(|(_, x)| x.is_ascii_digit());

        let Some(f) = itr.next() else {
            c += format!("{}{}", first.1.unwrap(), last.1.unwrap())
                .parse::<u32>()
                .unwrap();
            continue;
        };
        let l = itr.last().unwrap_or(f);

        if f.0 < first.0 {
            first = (f.0, f.1.to_string().parse().ok());
        }

        if l.0 >= last.0 {
            last = (l.0, l.1.to_string().parse().ok());
        }
        c += format!("{}{}", first.1.unwrap(), last.1.unwrap())
            .parse::<u32>()
            .unwrap();
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
