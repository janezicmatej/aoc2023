use std::{collections::HashMap, iter::once};

enum Tilt {
    North,
    West,
    South,
    East,
}

fn get_load(floor: &[Vec<char>]) -> usize {
    floor
        .iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().map(move |y| (i, y)))
        .filter(|x| *x.1 == 'O')
        .map(|x| floor.len() - x.0)
        .sum()
}

fn swap<T: Copy>(floor: &mut [Vec<T>], from: (usize, usize), to: (usize, usize)) {
    let a = floor[from.0][from.1];
    let b = floor[to.0][to.1];
    floor[from.0][from.1] = b;
    floor[to.0][to.1] = a;
}

fn tilt(floor: &mut Vec<Vec<char>>, tilt: Tilt) {
    let (inner, outer) = match tilt {
        Tilt::North | Tilt::South => (floor[0].len(), floor.len()),
        Tilt::West | Tilt::East => (floor.len(), floor[0].len()),
    };
    let inx_n = |(i, j)| (j, i);
    let inx_s = |(i, j)| (inner - 1 - j, i);
    let inx_w = |(i, j)| (i, j);
    let inx_e = |(i, j)| (i, inner - 1 - j);
    for i in 0..outer {
        let mut ptr = 0;
        for j in 0..inner {
            let ((ii, jj), (pi, pj)) = match tilt {
                Tilt::North => (inx_n((i, j)), inx_n((i, ptr))),
                Tilt::South => (inx_s((i, j)), inx_s((i, ptr))),
                Tilt::East => (inx_e((i, j)), inx_e((i, ptr))),
                Tilt::West => (inx_w((i, j)), inx_w((i, ptr))),
            };
            match floor[ii][jj] {
                'O' => {
                    swap(floor, (ii, jj), (pi, pj));
                    ptr += 1;
                }
                '#' => ptr = j + 1,
                _ => (),
            }
        }
    }
}

fn tilt_cycle(floor: &mut Vec<Vec<char>>) {
    use Tilt::*;
    tilt(floor, North);
    tilt(floor, West);
    tilt(floor, South);
    tilt(floor, East);
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut f: Vec<Vec<_>> = input.lines().map(|x| x.chars().collect()).collect();
    tilt(&mut f, Tilt::North);
    Some(get_load(&f))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut f: Vec<Vec<_>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut memo: HashMap<String, usize> = HashMap::new();

    for i in 1.. {
        tilt_cycle(&mut f);
        let repr = f
            .iter()
            .flat_map(|x| x.iter().chain(once(&'\n')))
            .collect::<String>();

        if let Some(ii) = memo.insert(repr, i) {
            let m = i - ii;
            let shift = (1_000_000_000 - ii) % m;

            for _ in 0..shift {
                tilt_cycle(&mut f);
            }
            break;
        }
    }

    Some(get_load(&f))
}

aoc::solution!(14);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 14)),
            Some(136)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 14)),
            Some(64)
        );
    }
}
