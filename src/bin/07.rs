use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Label {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl From<char> for Label {
    fn from(value: char) -> Self {
        use Label::*;
        match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            'X' => Joker,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct ParseHandError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    labels: [Label; 5],
}

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let labels = s
            .chars()
            .map(Label::from)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseHandError)?;
        Ok(Hand { labels })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let sf = HandType::from(*self);
        let so = HandType::from(*other);

        sf.cmp(&so).then({
            let mut c = Ordering::Equal;
            for i in 0..5 {
                c = self.labels[i].cmp(&other.labels[i]);
                if c != Ordering::Equal {
                    break;
                }
            }
            c
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

impl From<Hand> for HandType {
    fn from(value: Hand) -> Self {
        let mut map = HashMap::new();

        for c in value.labels {
            *map.entry(c).or_insert(0) += 1;
        }

        let joker = map.remove(&Label::Joker).unwrap_or(0);

        let is_n_k = |n| joker == n || map.values().filter(|&x| *x == n - joker).count() > 0;

        if is_n_k(5) {
            return Self::Five;
        }
        if is_n_k(4) {
            return Self::Four;
        }
        // full house
        if map.values().count() <= 2 {
            return Self::FullHouse;
        }
        if is_n_k(3) {
            return Self::Three;
        }
        // two pair
        if map.values().count() <= 3 {
            return Self::TwoPair;
        }
        if is_n_k(2) {
            return Self::Pair;
        }

        Self::HighCard
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut v = input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(f, s)| (f.parse::<Hand>().unwrap(), s.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    v.sort_by(|f, o| f.0.cmp(&o.0));

    Some(
        v.into_iter()
            .enumerate()
            .map(|(i, x)| (i as u32 + 1) * x.1)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v = input
        .replace('J', "X")
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(f, s)| (f.parse::<Hand>().unwrap(), s.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    v.sort_by(|f, o| f.0.cmp(&o.0));

    Some(
        v.into_iter()
            .enumerate()
            .map(|(i, x)| (i as u32 + 1) * x.1)
            .sum::<u32>(),
    )
}

aoc::solution!(7);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 7)),
            Some(6440)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 7)),
            Some(5905)
        );
    }
}
