struct MappedPart {
    number: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl MappedPart {
    fn is_adjacent(&self, line: usize, column: usize) -> bool {
        self.line.abs_diff(line) <= 1
            && (self.start.abs_diff(column) <= 1 || self.end.abs_diff(column) <= 1)
    }
}

fn build_map(input: &str) -> Vec<MappedPart> {
    let mut numbers = Vec::new();
    for (idl, line) in input.lines().enumerate() {
        let mut index = 0;
        for n in line.split(|c: char| !c.is_ascii_digit()) {
            if n.is_empty() {
                index += 1;
                continue;
            }

            let num: u32 = n.parse().unwrap();
            let num_len = n.len();

            numbers.push(MappedPart {
                number: num,
                line: idl,
                start: index,
                end: index + num_len - 1,
            });

            index += num_len + 1;
        }
    }

    numbers
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = build_map(input);

    let mut part_numbers = 0;

    for (idl, line) in input.lines().enumerate() {
        for (idc, _) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| !(*c == '.' || c.is_ascii_digit()))
        {
            part_numbers += numbers
                .iter()
                .filter(|mp| mp.is_adjacent(idl, idc))
                .map(|mp| mp.number)
                .sum::<u32>();
        }
    }

    Some(part_numbers)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = build_map(input);

    let mut gear_ratios = 0;

    for (idl, line) in input.lines().enumerate() {
        for (idc, _) in line.chars().enumerate().filter(|(_, c)| *c == '*') {
            let touching: Vec<_> = numbers
                .iter()
                .filter(|mp| mp.is_adjacent(idl, idc))
                .map(|mp| mp.number)
                .collect();
            if touching.len() == 2 {
                gear_ratios += touching.iter().product::<u32>();
            }
        }
    }

    Some(gear_ratios)
}

aoc::solution!(3);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 3)),
            Some(4361)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 3)),
            Some(467835)
        );
    }
}
