fn hash(s: &str) -> usize {
    let mut total = 0;
    for c in s.chars().map(|x| x as usize) {
        total += c;
        total *= 17;
        total %= 256;
    }
    total
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().flat_map(|x| x.split(',')).map(hash).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = vec![Vec::<(&str, usize)>::new(); 256];

    for instruction in input.lines().flat_map(|x| x.split(',')) {
        let (label, n) = instruction.split_once(|x| x == '-' || x == '=').unwrap();
        let hash = hash(label);
        let indexed_map = map.get_mut(hash).unwrap();

        if instruction.contains('-') {
            if let Some((i, _)) = indexed_map.iter().enumerate().find(|(_, &x)| x.0 == label) {
                indexed_map.remove(i);
            }
        } else {
            let nbr = n.parse().unwrap();
            if let Some((i, _)) = indexed_map.iter().enumerate().find(|(_, &x)| x.0 == label) {
                indexed_map[i] = (label, nbr);
            } else {
                indexed_map.push((label, nbr))
            }
        }
    }

    Some(
        map.into_iter()
            .enumerate()
            .flat_map(|(i, x)| {
                x.into_iter()
                    .enumerate()
                    .map(move |(j, y)| (i + 1) * (j + 1) * y.1)
            })
            .sum(),
    )
}

aoc::solution!(15);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 15)),
            Some(1320)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 15)),
            Some(145)
        );
    }
}
