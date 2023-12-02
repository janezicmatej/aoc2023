use std::str::FromStr;

use aoc::parsers::to_vec;

struct Game {
    id: u32,
    balls: Vec<(u32, String)>,
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Game ").unwrap();
        let (id, rest) = rest.split_once(':').unwrap();
        let id = id.parse().unwrap();

        let balls = rest
            .split([',', ';'])
            .map(|x| x.strip_prefix(' ').unwrap().split_once(' ').unwrap())
            .map(|(n, c)| (n.parse::<u32>().unwrap(), c.to_string()))
            .collect();

        Ok(Game { id, balls })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<Game> = to_vec(input, '\n');

    let mut id_sum = 0;

    'games: for g in games {
        for (n, c) in g.balls {
            match c.as_str() {
                "blue" => {
                    if n > 14 {
                        continue 'games;
                    }
                }
                "red" => {
                    if n > 12 {
                        continue 'games;
                    }
                }
                "green" => {
                    if n > 13 {
                        continue 'games;
                    }
                }
                _ => continue 'games,
            }
        }

        id_sum += g.id;
    }

    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = to_vec(input, '\n');

    let mut total_power = 0;

    for g in games {
        let power: u32 = ["blue", "red", "green"]
            .iter()
            .map(|&c| {
                g.balls
                    .iter()
                    .filter(|(_, cd)| c == cd.as_str())
                    .map(|(n, _)| n)
                    .max()
                    .unwrap()
            })
            .product();

        total_power += power;
    }

    Some(total_power)
}

aoc::solution!(2);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 2)), Some(8));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 2)),
            Some(2286)
        );
    }
}
