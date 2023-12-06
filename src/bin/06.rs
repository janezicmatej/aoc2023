use aoc::parsers::to_vec;

fn win_options((time, distance): (u64, u64)) -> u64 {
    let discriminant = ((time.pow(2) - 4 * distance) as f64).sqrt();

    let first_zero = (time as f64 - discriminant) / 2.0;
    let mut second_zero = (time as f64 + discriminant) / 2.0;

    if second_zero.fract() == 0.0 {
        second_zero -= 1.0
    }

    second_zero as u64 - first_zero as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let (upt, upd) = input.split_once('\n')?;
    let time: Vec<u64> = to_vec(upt.strip_prefix("Time:      ")?, ' ');
    let distance: Vec<u64> = to_vec(upd.strip_prefix("Distance:  ")?, ' ');

    Some(time.into_iter().zip(distance).map(win_options).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (upt, upd) = input.split_once('\n')?;
    let time: u64 = upt
        .strip_prefix("Time:      ")?
        .split(' ')
        .flat_map(|x| x.chars())
        .collect::<String>()
        .parse()
        .ok()?;
    let distance: u64 = upd
        .strip_prefix("Distance:  ")?
        .split(' ')
        .flat_map(|x| x.chars())
        .collect::<String>()
        .parse()
        .ok()?;

    Some(win_options((time, distance)))
}

aoc::solution!(6);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 6)),
            Some(288)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 6)),
            Some(71503)
        );
    }
}
