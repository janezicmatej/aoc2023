fn mirror_h(shape: &[Vec<char>], smudges: usize) -> Option<usize> {
    (1..shape.len()).find(|&i| {
        shape
            .iter()
            .skip(i)
            .zip(shape.iter().take(i).rev())
            .map(|(x, y)| {
                x.iter()
                    .zip(y.iter())
                    .map(|(xx, yy)| (xx != yy) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
            == smudges
    })
}

fn mirror_v(shape: &[Vec<char>], smudges: usize) -> Option<usize> {
    let shape: Vec<Vec<char>> = (0..shape[0].len())
        .map(|col| (0..shape.len()).map(|row| shape[row][col]).collect())
        .collect();

    mirror_h(&shape, smudges)
}

fn solve(input: &str, smudges: usize) -> usize {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|line| line.chars().collect()).collect())
        .map(|shape: Vec<Vec<char>>| {
            mirror_v(&shape, smudges).unwrap_or_default()
                + mirror_h(&shape, smudges).unwrap_or_default() * 100
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 1))
}

aoc::solution!(13);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 13)),
            Some(405)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 13)),
            Some(400)
        );
    }
}
