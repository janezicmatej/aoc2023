use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Gear {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct GearParseError;

impl TryFrom<&str> for Gear {
    type Error = GearParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Gear::*;
        Ok(match value {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => return Err(GearParseError),
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Comparator {
    Lt,
    Gt,
}

#[derive(Debug)]
struct ComparatorParseError;

impl TryFrom<char> for Comparator {
    type Error = ComparatorParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Comparator::*;
        Ok(match value {
            '<' => Lt,
            '>' => Gt,
            _ => return Err(ComparatorParseError),
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Resolver {
    Accepted,
    Rejected,
    Delegated(String),
}

impl From<&str> for Resolver {
    fn from(value: &str) -> Self {
        use Resolver::*;
        match value {
            "A" => Accepted,
            "R" => Rejected,
            x => Delegated(x.to_string()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Workflow {
    workflows: Vec<WorkflowInner>,
}

#[derive(Debug)]
struct ParseWorkflowError;

impl TryFrom<&str> for Workflow {
    type Error = ParseWorkflowError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            workflows: value.split(',').filter_map(|x| x.try_into().ok()).collect(),
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum WorkflowInner {
    Resolver(Resolver),
    Rule((Gear, Comparator, usize, Resolver)),
}

#[derive(Debug)]
struct ParseWorkflowInnerError;

impl TryFrom<&str> for WorkflowInner {
    type Error = ParseWorkflowInnerError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.contains(':') {
            return Ok(WorkflowInner::Resolver(Resolver::from(value)));
        }

        let (rest, resolver) = value.split_once(':').unwrap();
        let resolver = Resolver::from(resolver);

        let (gear, number) = rest.split_once(|x| x == '<' || x == '>').unwrap();
        let gear = Gear::try_from(gear).map_err(|_| ParseWorkflowInnerError)?;
        let number = number.parse().map_err(|_| ParseWorkflowInnerError)?;

        let comparator = if value.contains('<') { '<' } else { '>' };
        let comparator = Comparator::try_from(comparator).map_err(|_| ParseWorkflowInnerError)?;

        Ok(WorkflowInner::Rule((gear, comparator, number, resolver)))
    }
}

#[derive(Debug)]
struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
struct ParseXmasError;

impl FromStr for Xmas {
    type Err = ParseXmasError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').ok_or(ParseXmasError)?;
        let s = s.strip_suffix('}').ok_or(ParseXmasError)?;
        let xmas: Vec<_> = s
            .split(',')
            .filter_map(|x| x.split_once('='))
            .map(|x| x.1)
            .filter_map(|x| x.parse().ok())
            .collect();

        Ok(Self {
            x: xmas[0],
            m: xmas[1],
            a: xmas[2],
            s: xmas[3],
        })
    }
}

impl Index<Gear> for Xmas {
    type Output = usize;
    fn index(&self, index: Gear) -> &Self::Output {
        use Gear::*;
        match index {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }
}

impl IndexMut<Gear> for Xmas {
    fn index_mut(&mut self, index: Gear) -> &mut Self::Output {
        use Gear::*;
        match index {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }
}

impl Xmas {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn apply(&self, workflow: &Workflow) -> Resolver {
        for w in workflow.workflows.iter() {
            match w {
                WorkflowInner::Resolver(x) => {
                    return x.clone();
                }
                WorkflowInner::Rule((g, c, n, r)) => {
                    let is_match = match c {
                        Comparator::Gt => self[*g] > *n,
                        Comparator::Lt => self[*g] < *n,
                    };
                    if is_match {
                        return r.clone();
                    }
                }
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
struct XmasRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Index<Gear> for XmasRange {
    type Output = (usize, usize);
    fn index(&self, index: Gear) -> &Self::Output {
        use Gear::*;
        match index {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }
}

impl IndexMut<Gear> for XmasRange {
    fn index_mut(&mut self, index: Gear) -> &mut Self::Output {
        use Gear::*;
        match index {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }
}

impl XmasRange {
    fn size(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn divide(self, workflow: &Workflow) -> Vec<(Self, Resolver)> {
        let mut processed = Vec::new();
        let mut rest = vec![self];

        for w in workflow.workflows.iter() {
            let mut new_rest = Vec::new();

            while let Some(xmas_range) = rest.pop() {
                match w {
                    WorkflowInner::Resolver(r) => processed.push((xmas_range, r.clone())),
                    WorkflowInner::Rule((g, c, n, r)) => {
                        let compare = |x: usize, y: usize| match c {
                            Comparator::Gt => x > y,
                            Comparator::Lt => x < y,
                        };

                        let mut min_ok = usize::MAX;
                        let mut max_ok = usize::MIN;
                        let mut min_e = usize::MAX;
                        let mut max_e = usize::MIN;

                        for i in xmas_range[*g].0..=xmas_range[*g].1 {
                            if compare(i, *n) {
                                max_ok = if max_ok < i { i } else { max_ok };
                                min_ok = if min_ok > i { i } else { min_ok };
                            } else {
                                max_e = if max_e < i { i } else { max_e };
                                min_e = if min_e > i { i } else { min_e };
                            }
                        }

                        if min_e <= max_e {
                            let mut r = xmas_range;
                            r[*g] = (min_e, max_e);
                            new_rest.push(r);
                        }
                        if min_ok <= max_ok {
                            let mut p = xmas_range;
                            p[*g] = (min_ok, max_ok);
                            processed.push((p, r.clone()));
                        }
                    }
                }
            }

            rest = new_rest;
        }

        processed
    }
}

fn build_map(input: &str) -> HashMap<&str, Workflow> {
    let mut map = HashMap::new();
    for (s, w) in input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .filter_map(|x| x.strip_suffix('}'))
        .filter_map(|x| x.split_once('{'))
        .filter_map(|(s, w)| Workflow::try_from(w).ok().map(|x| (s, x)))
    {
        map.insert(s, w);
    }

    map
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = build_map(input);
    let xmas_vec: Vec<Xmas> = input
        .split("\n\n")
        .nth(1)?
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut total = 0;

    'outer: for xmas in xmas_vec.into_iter() {
        let mut workflow = &map["in"];
        'apply: loop {
            match xmas.apply(workflow) {
                Resolver::Accepted => break 'apply,
                Resolver::Rejected => continue 'outer,
                Resolver::Delegated(x) => workflow = &map[x.as_str()],
            }
        }

        total += xmas.sum();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = build_map(input);

    let mut finished = Vec::new();
    let mut stack = vec![(
        XmasRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        Resolver::Delegated("in".to_string()),
    )];

    while let Some((xmas_range, resolver)) = stack.pop() {
        match resolver {
            Resolver::Accepted => finished.push(xmas_range),
            Resolver::Rejected => {
                continue;
            }
            Resolver::Delegated(x) => {
                let workflow = &map[x.as_str()];
                stack.append(&mut xmas_range.divide(workflow))
            }
        }
    }

    Some(finished.iter().map(XmasRange::size).sum())
}
aoc::solution!(19);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file("examples", 19)),
            Some(19114)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file("examples", 19)),
            Some(167409079868000)
        );
    }
}
